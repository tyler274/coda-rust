//! Duet's two tab roots, You and Shared. SwiftUI remains the shipping iPhone app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

mod account;
#[cfg(target_os = "ios")]
mod apple_health;
mod approval;
mod browser_identity;
mod export;
mod preview;
mod publication;
mod records;
mod resolution;
mod roots;
mod selection;
pub use account::{DuetRoot, PatientSession};
pub use approval::proposal_save;
pub use preview::{SavedPreview, saved_preview};

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use dioxus::prelude::*;
use duet_client::DuetClient;
use duet_notify::{AlertSnapshot, AlertState, PlatformAlerts};
use roots::{
    ALLOW_NOTIFICATIONS, CONNECTING, DoctorPage, LIBRARY_EMPTY, LIBRARY_UNAVAILABLE, LibraryKind,
    LibraryRecord, LibraryRoot, OPEN_NOTIFICATION_SETTINGS, TRY_AGAIN, doctor_page,
    doctor_plan_section, library_root,
};
use ui::{
    ActionRow, Button, ButtonVariant, Disclosure, Icon, IconName, NavigationControl, TabBar,
    TryAgain,
};

const TABS: [&str; 2] = ["You", "Shared"];

const DUET_STYLES: &str = include_str!("../assets/duet.css");

pub use roots::{api_base_url, api_bearer};

#[component]
fn PrivateRoot(base_url: String, bearer: String) -> Element {
    let mut tab = use_signal(|| 0usize);
    let mut alerts = use_signal(|| PlatformAlerts.snapshot());
    rsx! {
        ui::Stylesheet { app: DUET_STYLES }
        main { class: "duet theme-duet",
            div { class: "duet-page",
                div { hidden: tab() != 0,
                    YouTab {
                        base_url: base_url.clone(),
                        bearer: bearer.clone(),
                        alerts: alerts(),
                        on_allow_alerts: move |_| {
                            PlatformAlerts.request_access();
                            alerts.set(PlatformAlerts.snapshot());
                        },
                        on_open_settings: move |_| PlatformAlerts.open_settings(),
                    }
                }
                div { hidden: tab() != 1, SharedTab { base_url: base_url.clone(), bearer: bearer.clone() } }
            }
            TabBar {
                label: "Sections",
                labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                icons: vec![IconName::User, IconName::Users],
                selected: tab(),
                on_select: move |index| {
                    tab.set(index);
                    if index == 0 {
                        alerts.set(PlatformAlerts.snapshot());
                    }
                },
            }
        }
    }
}

fn use_deferred_status(loading: Signal<bool>) -> bool {
    let mut shown = use_signal(|| false);
    let tickets = use_hook(|| Rc::new(Cell::new(0u64)));
    let tickets_effect = tickets.clone();
    use_effect(move || {
        let is_loading = loading();
        let next = tickets_effect.get() + 1;
        tickets_effect.set(next);
        if !is_loading {
            shown.set(false);
            return;
        }
        let tickets_effect = tickets_effect.clone();
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(300)).await;
            if tickets_effect.get() == next {
                shown.set(true);
            }
        });
    });
    shown() && loading()
}

#[component]
fn TabHeading(name: String, loading: Signal<bool>) -> Element {
    let waiting = use_deferred_status(loading);
    rsx! {
        div { class: "duet-heading",
            h1 { "{name}" }
            p { class: "duet-status", if waiting { "{CONNECTING}" } }
        }
    }
}

fn open_client(base_url: String, bearer: String) -> Result<DuetClient, String> {
    if let Some(session) = try_consume_context::<PatientSession>() {
        if !session.retention.session().is_current(&session.fence) {
            return Err("Account session ended".into());
        }
        let client = DuetClient::new(base_url, session.access_token.peek().clone(), "0", "0.1.0")?;
        let sender = consume_context::<account::RefusalSender>();
        return Ok(client.bind_session(session.retention.session(), session.fence, sender.0));
    }
    DuetClient::new(base_url, bearer, "0", "0.1.0")
}

fn optional_text(value: &str) -> Option<String> {
    let text = value.trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_owned())
    }
}

fn doctor_draft_save(
    collection: &duet_wire::models::DoctorCollection,
    name: &str,
    birth: &str,
    note: &str,
    cover: &str,
    questions: &[String],
    kept: &[uuid::Uuid],
) -> duet_wire::models::DoctorPageSave {
    // Preserve the reviewed ordered selection, including proposal-only members.
    // The server checks exact membership against generation/proposal identity.
    let item_ids = kept.to_vec();
    let mut page = collection
        .page
        .as_ref()
        .map(|page| (**page).clone())
        .unwrap_or_default();
    page.name = Some(optional_text(name));
    page.date_of_birth = Some(
        optional_text(birth)
            .and_then(|text| chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d").ok()),
    );
    page.cover_note = Some(optional_text(cover));
    let questions: Vec<String> = questions
        .iter()
        .map(|question| question.trim().to_owned())
        .filter(|question| !question.is_empty())
        .collect();
    page.questions = if questions.is_empty() {
        None
    } else {
        Some(questions)
    };
    let mut save =
        duet_wire::models::DoctorPageSave::new(collection.note_generation, item_ids, page);
    save.note = Some(optional_text(note));
    save
}

fn field_text(value: &Option<Option<String>>) -> String {
    value
        .as_ref()
        .and_then(|inner| inner.clone())
        .unwrap_or_default()
}

fn doctor_draft_ready(birth: &str, cover: &str, questions: &[String]) -> bool {
    let birth = birth.trim();
    let birth_ok = birth.is_empty() || chrono::NaiveDate::parse_from_str(birth, "%Y-%m-%d").is_ok();
    let questions_ok = questions.iter().all(|question| !question.trim().is_empty());
    birth_ok && questions_ok && cover.chars().count() <= 500
}

#[derive(Clone, Copy, PartialEq)]
enum YouEditor {
    Medication,
    Note,
}

#[derive(Clone)]
struct EvidenceEdit {
    id: String,
    kind: LibraryKind,
    title: String,
}

#[component]
fn SharedTab(base_url: String, bearer: String) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut doctor = use_signal(|| None);
    let mut doctor_open = use_signal(|| false);
    let mut saving_page = use_signal(|| false);
    let mut page_failed = use_signal(|| false);
    let patient = use_context::<PatientSession>();
    let draft_owner = patient.retention.clone();
    let draft_fence = patient.fence.clone();
    let mut retained = use_signal(|| draft_owner.recover(&draft_fence));
    let mut retention_error = use_signal(String::new);
    let mut draft_base = use_signal(|| None::<duet_wire::models::DoctorCollection>);
    let mut draft_uncertain = use_signal(|| false);
    let mut confirm_discard = use_signal(|| false);
    let mut doctor_editing = use_signal(|| false);
    let mut proposal_review = use_signal(|| None::<uuid::Uuid>);
    let mut draft_proposal = use_signal(|| None::<uuid::Uuid>);
    let mut confirmed_medications = use_signal(Vec::<uuid::Uuid>::new);
    let mut doctor_name = use_signal(String::new);
    let mut doctor_birth = use_signal(String::new);
    let mut doctor_note = use_signal(String::new);
    let mut doctor_cover = use_signal(String::new);
    let mut doctor_questions = use_signal(Vec::<String>::new);
    let mut doctor_items = use_signal(Vec::<(uuid::Uuid, String)>::new);
    let mut doctor_kept = use_signal(Vec::<bool>::new);
    let persist_owner = draft_owner.clone();
    let persist_fence = draft_fence.clone();
    use_effect(move || {
        if !doctor_editing() || saving_page() || draft_uncertain() {
            return;
        }
        let Some(base) = draft_base() else {
            return;
        };
        let kept = doctor_items()
            .into_iter()
            .zip(doctor_kept())
            .filter_map(|(item, keep)| keep.then_some(item.0))
            .collect::<Vec<_>>();
        let mut save = doctor_draft_save(
            &base,
            &doctor_name(),
            &doctor_birth(),
            &doctor_note(),
            &doctor_cover(),
            &doctor_questions(),
            &kept,
        );
        save.confirmed_medication_ids = Some(confirmed_medications());
        save.proposal_id = Some(draft_proposal());
        let draft = duet_client::retention::DoctorDraft {
            base,
            save,
            uncertain_save: draft_uncertain(),
        };
        match persist_owner.save(&persist_fence, draft.clone()) {
            Ok(()) => {
                retained.set(Ok(duet_client::retention::DraftRecovery::Draft(Box::new(
                    draft,
                ))));
                retention_error.set(String::new());
            }
            Err(error) => retention_error.set(error),
        }
    });
    let recover_owner = draft_owner.clone();
    let recover_fence = draft_fence.clone();
    let mut restore_draft = move |_| {
        let recovery = recover_owner.recover(&recover_fence);
        retained.set(recovery.clone());
        if let Ok(duet_client::retention::DraftRecovery::Draft(draft)) = recovery {
            let page = &draft.save.page;
            doctor_name.set(field_text(&page.name));
            doctor_birth.set(
                page.date_of_birth
                    .as_ref()
                    .and_then(|d| *d)
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
            );
            doctor_note.set(field_text(&draft.save.note));
            doctor_cover.set(field_text(&page.cover_note));
            doctor_questions.set(page.questions.clone().unwrap_or_default());
            let reviewed_items = draft
                .base
                .proposal
                .as_ref()
                .and_then(|p| p.as_ref())
                .filter(|p| Some(p.id) == draft.save.proposal_id.flatten())
                .map(|p| &p.items)
                .or(draft.base.items.as_ref());
            let items = reviewed_items
                .into_iter()
                .flatten()
                .map(|i| (i.id, i.title.clone()))
                .collect::<Vec<_>>();
            doctor_kept.set(
                items
                    .iter()
                    .map(|i| draft.save.item_ids.contains(&i.0))
                    .collect(),
            );
            doctor_items.set(items);
            doctor.set(Some(draft.base.clone()));
            draft_base.set(Some(draft.base));
            confirmed_medications.set(draft.save.confirmed_medication_ids.unwrap_or_default());
            draft_proposal.set(draft.save.proposal_id.flatten());
            draft_uncertain.set(draft.uncertain_save);
            doctor_open.set(true);
            doctor_editing.set(true);
        }
    };
    let mut edit_recovered = restore_draft.clone();
    let discard_owner = draft_owner.clone();
    let discard_fence = draft_fence.clone();
    let discard_draft = move |_| match discard_owner.discard(&discard_fence) {
        Ok(()) => {
            doctor_editing.set(false);
            draft_base.set(None);
            retained.set(Ok(duet_client::retention::DraftRecovery::Empty));
            confirm_discard.set(false);
            retention_error.set(String::new());
        }
        Err(error) => retention_error.set(error),
    };
    let mut confirm_fresh = use_signal(|| false);
    let mut fresh_failed = use_signal(|| false);
    let export_url = base_url.clone();
    let page_url = base_url.clone();
    let page_bearer = bearer.clone();
    let reconcile_owner = patient.retention.clone();
    let reconcile_fence = patient.fence.clone();
    let workspace_url = base_url.clone();
    let workspace_token = bearer.clone();
    let workspace_owner = patient.retention.clone();
    let workspace_fence = patient.fence.clone();
    let workspace = use_resource(move || {
        let _attempt = attempt();
        let url = workspace_url.clone();
        let bearer = workspace_token.clone();
        let recovery = workspace_owner.recover(&workspace_fence);
        async move {
            let api = open_client(url, bearer)
                .map_err(|_| duet_client::doctor::DoctorError::Unauthorized)?;
            let current = api.doctor_workspace().await?;
            let receipt = if let Ok(duet_client::retention::DraftRecovery::Draft(draft)) = recovery
            {
                if draft.uncertain_save {
                    Some(
                        api.doctor_save_receipt(draft.save.expected_generation)
                            .await,
                    )
                } else {
                    None
                }
            } else {
                None
            };
            Ok::<_, duet_client::doctor::DoctorError>((current, receipt))
        }
    });
    use_effect(move || {
        if !reconcile_owner.session().is_current(&reconcile_fence) {
            return;
        }
        if let Some(Ok((collection, receipt))) = workspace() {
            let recovery = retained.peek().clone();
            if let Ok(duet_client::retention::DraftRecovery::Draft(draft)) = recovery
                && draft.uncertain_save
            {
                match receipt {
                        Some(Ok(saved)) if draft.reconcile(&saved) == duet_client::retention::SaveReadback::Saved => {
                            if reconcile_owner.discard(&reconcile_fence).is_ok() { retained.set(Ok(duet_client::retention::DraftRecovery::Empty)); draft_uncertain.set(false); doctor_editing.set(false); }
                        },
                        Some(Err(duet_client::doctor::DoctorError::NotFound)) if draft.reconcile(&collection) == duet_client::retention::SaveReadback::RetryOriginal => {
                            retention_error.set("No receipt yet and the base generation is unchanged. Retry only the retained original save.".into());
                        },
                        _ => retention_error.set("The exact save receipt could not confirm your request. Your draft is retained; no mutation was replayed.".into()),
                    }
            }
            doctor.set(Some(collection));
        }
    });
    use_effect(move || loading.set(workspace().is_none()));
    let shown = if doctor_open() {
        doctor().as_ref().map(doctor_page)
    } else {
        None
    };
    let fresh_url = page_url.clone();
    let fresh_bearer = page_bearer.clone();
    let save_page = move |_| {
        if saving_page()
            || !doctor_draft_ready(&doctor_birth(), &doctor_cover(), &doctor_questions())
        {
            return;
        }
        let Some(collection) = draft_base() else {
            return;
        };
        if draft_uncertain() {
            page_failed.set(true);
            return;
        }
        let kept = doctor_items()
            .into_iter()
            .zip(doctor_kept())
            .filter_map(|(item, keep)| keep.then_some(item.0))
            .collect::<Vec<_>>();
        let mut save = doctor_draft_save(
            &collection,
            &doctor_name(),
            &doctor_birth(),
            &doctor_note(),
            &doctor_cover(),
            &doctor_questions(),
            &kept,
        );
        save.confirmed_medication_ids = Some(
            confirmed_medications()
                .into_iter()
                .filter(|id| save.item_ids.contains(id))
                .collect(),
        );
        save.proposal_id = Some(draft_proposal());
        let pending = duet_client::retention::DoctorDraft {
            base: collection,
            save: save.clone(),
            uncertain_save: true,
        };
        if let Err(error) = draft_owner.save(&draft_fence, pending.clone()) {
            retention_error.set(error);
            // Browser/non-native builds may save online but must not claim persistence.
            if cfg!(any(target_os = "ios", target_os = "android")) {
                return;
            }
        }
        retained.set(Ok(duet_client::retention::DraftRecovery::Draft(Box::new(
            pending,
        ))));
        draft_uncertain.set(true);
        let save_owner = draft_owner.clone();
        let save_fence = draft_fence.clone();
        saving_page.set(true);
        page_failed.set(false);
        let base_url = page_url.clone();
        let bearer = page_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.save_doctor_page(save).await,
                Err(_) => Err(String::new()),
            };
            if !save_owner.session().is_current(&save_fence) {
                return;
            }
            match saved {
                Ok(collection) => {
                    doctor.set(Some(collection));
                    page_failed.set(false);
                    doctor_editing.set(false);
                    draft_uncertain.set(false);
                    match save_owner.discard(&save_fence) {
                        Ok(()) => retained.set(Ok(duet_client::retention::DraftRecovery::Empty)),
                        Err(error) => retention_error.set(error),
                    }
                }
                Err(_) => page_failed.set(true),
            }
            saving_page.set(false);
        });
    };
    let proposal_owner = patient.retention.clone();
    let proposal_fence = patient.fence.clone();
    let proposal_url = export_url.clone();
    let proposal_token = patient.access_token;
    let approve_proposal = move |_| {
        if saving_page() {
            return;
        }
        let Some(collection) = doctor() else {
            return;
        };
        let has_draft = !matches!(retained(), Ok(duet_client::retention::DraftRecovery::Empty));
        if collection
            .proposal
            .as_ref()
            .and_then(|p| p.as_ref())
            .map(|p| p.id)
            != proposal_review()
        {
            page_failed.set(true);
            return;
        }
        let save = match proposal_save(&collection, &confirmed_medications(), has_draft) {
            Ok(save) => save,
            Err(error) => {
                retention_error.set(error.into());
                return;
            }
        };
        let pending = duet_client::retention::DoctorDraft {
            base: collection,
            save: save.clone(),
            uncertain_save: true,
        };
        if let Err(error) = proposal_owner.save(&proposal_fence, pending.clone()) {
            retention_error.set(error);
            return;
        }
        retained.set(Ok(duet_client::retention::DraftRecovery::Draft(Box::new(
            pending,
        ))));
        saving_page.set(true);
        draft_uncertain.set(true);
        let owner = proposal_owner.clone();
        let fence = proposal_fence.clone();
        let url = proposal_url.clone();
        spawn(async move {
            let result = match open_client(url, proposal_token.peek().clone()) {
                Ok(api) => api.save_doctor_page(save).await,
                Err(error) => Err(error),
            };
            if !owner.session().is_current(&fence) {
                return;
            }
            match result {
                Ok(saved) => {
                    doctor.set(Some(saved));
                    proposal_review.set(None);
                    draft_uncertain.set(false);
                    if owner.discard(&fence).is_ok() {
                        retained.set(Ok(duet_client::retention::DraftRecovery::Empty));
                    }
                }
                Err(_) => page_failed.set(true),
            }
            saving_page.set(false);
        });
    };
    let reject_patient = patient.clone();
    let reject_url = export_url.clone();
    let reject_proposal = move |_| {
        if saving_page() {
            return;
        }
        let Some(current) = doctor() else {
            return;
        };
        let Some(id) = proposal_review() else {
            return;
        };
        saving_page.set(true);
        let session = reject_patient.clone();
        let url = reject_url.clone();
        spawn(async move {
            let result = match open_client(url, session.access_token.peek().clone()) {
                Ok(api) => {
                    api.reject_doctor_proposal(id, current.note_generation)
                        .await
                }
                Err(_) => Err(duet_client::doctor::DoctorError::Unauthorized),
            };
            if !session.retention.session().is_current(&session.fence) {
                return;
            }
            match result {
                Ok(collection) => {
                    doctor.set(Some(collection));
                    proposal_review.set(None);
                }
                Err(e) => {
                    retention_error.set(e.to_string());
                    attempt.set(attempt() + 1);
                }
            }
            saving_page.set(false);
        });
    };
    let retry_session = patient.clone();
    let retry_url = export_url.clone();
    let retry_exact_save = move |_| {
        if saving_page() {
            return;
        }
        let Ok(duet_client::retention::DraftRecovery::Draft(draft)) = retained() else {
            return;
        };
        if !draft.uncertain_save {
            return;
        }
        saving_page.set(true);
        let session = retry_session.clone();
        let url = retry_url.clone();
        spawn(async move {
            let result = match open_client(url, session.access_token.peek().clone()) {
                Ok(api) => api.save_doctor_workspace(&draft.save).await,
                Err(_) => Err(duet_client::doctor::DoctorError::Unauthorized),
            };
            if !session.retention.session().is_current(&session.fence) {
                return;
            }
            match result {
                Ok(saved)
                    if draft.reconcile(&saved) == duet_client::retention::SaveReadback::Saved =>
                {
                    if session.retention.discard(&session.fence).is_ok() {
                        retained.set(Ok(duet_client::retention::DraftRecovery::Empty));
                        draft_uncertain.set(false);
                        doctor_editing.set(false);
                    }
                    // Read current page separately; acknowledgement can be an older exact receipt.
                    attempt.set(attempt() + 1);
                }
                Ok(_) => retention_error.set(
                    "Unexpected save acknowledgement; the original draft remains retained.".into(),
                ),
                Err(e) => retention_error.set(e.to_string()),
            }
            saving_page.set(false);
        });
    };
    let doctor_source_url = export_url.clone();
    let source_blocked = !matches!(retained(), Ok(duet_client::retention::DraftRecovery::Empty))
        || !matches!(
            patient.retention.pending_operation(&patient.fence),
            Ok(None)
        );
    let fresh_owner = patient.retention.clone();
    let fresh_fence = patient.fence.clone();
    let start_fresh = move |_| {
        if saving_page() {
            return;
        }
        let operation = match fresh_owner.pending_operation(&fresh_fence) {
            Ok(Some(op @ duet_client::doctor::PendingDoctorOperation::Clear { .. })) => op,
            Ok(None) => {
                let Some(current) = doctor() else {
                    return;
                };
                duet_client::doctor::PendingDoctorOperation::Clear {
                    operation_id: uuid::Uuid::new_v4(),
                    expected_generation: current.note_generation.into(),
                }
            }
            _ => {
                retention_error
                    .set("Resolve the other pending operation before Start fresh".into());
                return;
            }
        };
        if let Err(error) = fresh_owner.retain_operation(&fresh_fence, &operation) {
            retention_error.set(error);
            return;
        }
        let duet_client::doctor::PendingDoctorOperation::Clear {
            operation_id,
            expected_generation,
        } = operation
        else {
            return;
        };
        saving_page.set(true);
        fresh_failed.set(false);
        let base_url = fresh_url.clone();
        let bearer = fresh_bearer.clone();
        let owner = fresh_owner.clone();
        let fence = fresh_fence.clone();
        spawn(async move {
            let cleared = match open_client(base_url, bearer) {
                Ok(api) => match api
                    .clear_doctor_workspace(operation_id, expected_generation)
                    .await
                {
                    Ok(()) => api.doctor_workspace().await.map_err(|e| e.to_string()),
                    Err(e) => {
                        if e == duet_client::doctor::DoctorError::Conflict {
                            let _ = owner.mark_operation_conflict(
                                &fence,
                                &duet_client::doctor::PendingDoctorOperation::Clear {
                                    operation_id,
                                    expected_generation,
                                },
                            );
                        }
                        Err(e.to_string())
                    }
                },
                Err(_) => Err(String::new()),
            };
            if !owner.session().is_current(&fence) {
                return;
            }
            match cleared {
                Ok(current) => {
                    if let Err(error) = owner.finish_operation(&fence) {
                        retention_error.set(error);
                        saving_page.set(false);
                        return;
                    }
                    match owner.discard(&fence) {
                        Ok(()) => retained.set(Ok(duet_client::retention::DraftRecovery::Empty)),
                        Err(error) => retention_error.set(error),
                    }
                    draft_base.set(None);
                    draft_uncertain.set(false);
                    doctor.set(Some(current));
                    attempt.set(attempt() + 1);
                    doctor_editing.set(false);
                    confirm_fresh.set(false);
                    fresh_failed.set(false);
                }
                Err(_) => fresh_failed.set(true),
            }
            saving_page.set(false);
        });
    };
    rsx! {
        resolution::OperationRecovery { base_url: doctor_source_url.clone(), on_released: move |_| { attempt.set(attempt() + 1); doctor_open.set(false); } }
        selection::DeviceSources { base_url: doctor_source_url.clone(), blocked: source_blocked, on_selected: move |collection| { doctor.set(Some(collection)); doctor_open.set(true); }, on_recheck: move |_| attempt.set(attempt() + 1) }
        selection::DoctorSources { base_url: doctor_source_url, blocked: source_blocked, on_selected: move |collection| { doctor.set(Some(collection)); doctor_open.set(true); } }
        if let Some(Err(error)) = workspace() {
            div { role: "alert",
                p { "For my doctor: {error}" }
                TryAgain { label: TRY_AGAIN, on_retry: move |_| attempt.set(attempt() + 1) }
            }
        }
        if doctor().is_some() && !doctor_open() { NavigationControl { onclick: move |_| doctor_open.set(true), "Open For my doctor" } }
        if let Some(proposal) = doctor().and_then(|c| c.proposal.flatten()) {
            if proposal_review() != Some(proposal.id) {
                Button { onclick: move |_| { proposal_review.set(Some(proposal.id)); confirmed_medications.set(Vec::new()); }, "Review proposed doctor page" }
            } else {
                section {
                    h2 { "Proposed page — not saved" }
                    p { "{field_text(&proposal.page.name)}" }
                    if let Some(day) = proposal.page.date_of_birth.flatten() { p { "Date of birth: {day}" } }
                    p { "{proposal.note.clone().unwrap_or_default()}" }
                    for question in proposal.page.questions.iter().flatten() { p { "{question}" } }
                    p { "Cover note (not included in PDF): {field_text(&proposal.page.cover_note)}" }
                    for item in &proposal.items {
                        article {
                            p { "{item.title}" }
                            if let Some(entry) = item.page_entry.as_ref().and_then(|entry| entry.as_ref()) {
                                for line in &entry.lines { p { "{line}" } }
                            } else { p { "{item.detail}" } }
                            if approval::is_medication(item) {
                                label {
                                    input { r#type: "checkbox", checked: confirmed_medications().contains(&item.id), onchange: { let id = item.id; move |event: Event<FormData>| { let mut ids = confirmed_medications(); ids.retain(|value| *value != id); if event.checked() { ids.push(id); } confirmed_medications.set(ids); } } }
                                    "I confirm I take this medication"
                                }
                            }
                        }
                    }
                    ActionRow {
                        Button { variant: ButtonVariant::Primary, disabled: saving_page() || draft_uncertain(), onclick: approve_proposal, "Approve and save reviewed proposal" }
                        Button { disabled: saving_page() || draft_uncertain(), onclick: reject_proposal, "Reject reviewed proposal" }
                        Button { onclick: move |_| proposal_review.set(None), "Close review without saving" }
                    }
                }
            }
        }
        if !retention_error().is_empty() { p { role: "alert", "Draft protection: {retention_error}. Keep this screen open until your page is saved." } }
        if let Err(error) = retained() { p { role: "alert", "Protected draft recovery is unavailable: {error}. No retained draft has been discarded." } }
        if matches!(retained(), Ok(duet_client::retention::DraftRecovery::Damaged)) {
            p { role: "alert", "The protected draft is damaged. It has not been overwritten. Discard it only if you no longer need it." }
        }
        if matches!(retained(), Ok(duet_client::retention::DraftRecovery::Draft(_))) && !doctor_editing() {
            Button { onclick: move |_| restore_draft(()), "Recover unsaved doctor draft" }
        }
        if draft_uncertain() {
            Button { disabled: saving_page(), onclick: retry_exact_save, "Retry exact original save" }
            p { role: "alert", "The last save may have succeeded. Your exact draft is retained. Compare it with the saved page before retrying; do not silently rebase or duplicate the save." }
            Button { onclick: move |_| attempt.set(attempt() + 1), "Reload saved page to reconcile" }
        }
        if !matches!(retained(), Ok(duet_client::retention::DraftRecovery::Empty)) {
            Button { danger: true, disabled: saving_page(), onclick: move |_| confirm_discard.set(true), "Discard local draft…" }
        }
        if confirm_discard() {
            p { "Discard this unsaved local draft? This does not delete the approved server page." }
            ActionRow {
                Button { variant: ButtonVariant::Primary, danger: true, onclick: discard_draft, "Confirm discard" }
                Button { onclick: move |_| confirm_discard.set(false), "Keep draft" }
            }
        }
        if let Some(shown) = shown {
            TabHeading { name: "For my doctor".to_owned(), loading }
            NavigationControl {
                aria_label: "Close page",
                onclick: move |_| {
                    doctor_open.set(false);
                    doctor_editing.set(false);
                    confirm_fresh.set(false);
                    page_failed.set(false);
                    fresh_failed.set(false);
                },
                Icon { name: IconName::ArrowLeft }
                "Close"
            }
            match shown {
                DoctorPage::Lines(entries) if !doctor_editing() => rsx! {
                    div { class: "duet-thread",
                        for entry in entries {
                            p { class: "card-text", "{entry.title}" }
                            if let Some(detail) = entry.detail {
                                p { class: "record-meta", "{detail}" }
                            }
                        }
                    }
                    DoctorLetter { collection: doctor(), base_url: export_url.clone() }
                },
                DoctorPage::Empty if !doctor_editing() => rsx! {
                    p { class: "empty", "{LIBRARY_EMPTY}" }
                    DoctorLetter { collection: doctor(), base_url: export_url.clone() }
                },
                _ => rsx! {},
            }
            if doctor_editing() && !draft_uncertain() {
                if let Some(base) = draft_base() {
                    for item in base.items.iter().flatten().filter(|item| approval::is_medication(item)) {
                        label {
                            input { r#type: "checkbox", checked: confirmed_medications().contains(&item.id), onchange: { let id = item.id; move |event: Event<FormData>| { let mut ids = confirmed_medications(); ids.retain(|value| *value != id); if event.checked() { ids.push(id); } confirmed_medications.set(ids); } } }
                            "I confirm I take {item.title}"
                        }
                    }
                }
                DoctorEditor {
                    name: doctor_name,
                    birth: doctor_birth,
                    note: doctor_note,
                    cover: doctor_cover,
                    questions: doctor_questions,
                    items: doctor_items(),
                    kept: doctor_kept,
                    saving: saving_page,
                    failed: page_failed,
                    on_save: save_page,
                }
            }
            if !doctor_editing() && confirm_fresh() {
                p { class: "status-title", "Start fresh?" }
                p { class: "status-detail", "This clears your words, questions, chosen information and cover note. Save a PDF first if you want a copy." }
                if fresh_failed() {
                    p { class: "empty", role: "alert", "Duet couldn’t confirm that For my doctor was cleared. Try again." }
                }
                ActionRow {
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: saving_page(),
                        onclick: start_fresh,
                        "Start fresh"
                    }
                    Button {
                        disabled: saving_page(),
                        onclick: move |_| confirm_fresh.set(false),
                        "Cancel"
                    }
                }
            }
            if !doctor_editing() && !confirm_fresh() {
                ActionRow {
                    Button {
                        disabled: saving_page(),
                        onclick: move |_| {
                            let Some(collection) = doctor() else {
                                return;
                            };
                            if matches!(retained(), Ok(duet_client::retention::DraftRecovery::Draft(_))) { edit_recovered(()); return; }
                            if matches!(retained(), Ok(duet_client::retention::DraftRecovery::Damaged)) { return; }
                            confirmed_medications.set(collection.items.iter().flatten().filter(|item| approval::is_medication(item) && item.medication_confirmed == Some(true)).map(|item| item.id).collect());
                            draft_proposal.set(None);
                            draft_base.set(Some(collection.clone()));
                            draft_uncertain.set(false);
                            let page = collection.page.as_ref().map(|page| page.as_ref());
                            doctor_name.set(page.map(|page| field_text(&page.name)).unwrap_or_default());
                            doctor_birth.set(
                                page.and_then(|page| page.date_of_birth.as_ref())
                                    .and_then(|day| day.as_ref())
                                    .map(|day| day.format("%Y-%m-%d").to_string())
                                    .unwrap_or_default(),
                            );
                            doctor_note.set(field_text(&collection.note));
                            doctor_cover.set(page.map(|page| field_text(&page.cover_note)).unwrap_or_default());
                            doctor_questions.set(page.and_then(|page| page.questions.clone()).unwrap_or_default());
                            let items = collection
                                .items
                                .iter()
                                .flatten()
                                .map(|item| (item.id, item.title.clone()))
                                .collect::<Vec<_>>();
                            doctor_kept.set(vec![true; items.len()]);
                            doctor_items.set(items);
                            page_failed.set(false);
                            doctor_editing.set(true);
                        },
                        aria_label: "Edit page",
                        "Edit page"
                    }
                    Button {
                        disabled: saving_page(),
                        onclick: move |_| {
                            fresh_failed.set(false);
                            confirm_fresh.set(true);
                        },
                        "Start fresh"
                    }
                }
            }
        } else {
            TabHeading { name: "Shared".to_owned(), loading }
        }
    }
}

fn evidence_entity(kind: LibraryKind) -> duet_wire::models::evidence_item::Entity {
    match kind {
        LibraryKind::HealthEvidence => duet_wire::models::evidence_item::Entity::HealthEvidence,
        LibraryKind::ClinicalNote => duet_wire::models::evidence_item::Entity::ClinicalNote,
    }
}

#[component]
fn EvidenceRow(
    record: LibraryRecord,
    base_url: String,
    bearer: String,
    editing: Signal<Option<EvidenceEdit>>,
    notice: Signal<Option<String>>,
    attempt: Signal<u32>,
    saving: Signal<bool>,
) -> Element {
    let title = record.title.clone();
    let summary = record.summary.clone();
    let when = record.when.clone();
    let source = record.source.clone();
    let correct_label = format!("Correct {title}");
    let delete_label = format!("Delete {title}");
    let kind = record.kind;
    let correct_id = record.id.clone();
    let correct_title = title.clone();
    let start_edit = move |_| {
        notice.set(None);
        editing.set(Some(EvidenceEdit {
            id: correct_id.clone(),
            kind,
            title: correct_title.clone(),
        }));
    };
    let delete_id = record.id;
    let delete_url = base_url.clone();
    let delete_bearer = bearer.clone();
    let remove = move |_| {
        if saving() {
            return;
        }
        saving.set(true);
        notice.set(None);
        let id = delete_id.clone();
        let base_url = delete_url.clone();
        let bearer = delete_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.delete_evidence(&id, evidence_entity(kind)).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    notice.set(Some("Removed from your records.".to_owned()));
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    notice.set(Some("This record couldn’t be removed.".to_owned()));
                }
            }
            saving.set(false);
        });
    };
    rsx! {
        p { class: "card-text", "{title}" }
        if let Some(summary) = summary {
            p { class: "record-meta", "{summary}" }
        }
        if let Some(when) = when {
            p { class: "record-meta", "{when}" }
        }
        if let Some(source) = source {
            p { class: "record-meta", "{source}" }
        }
        ActionRow {
            Button {
                compact: true,
                aria_label: "{correct_label}",
                onclick: start_edit,
                "Correct"
            }
            Button {
                compact: true,
                danger: true,
                aria_label: "{delete_label}",
                onclick: remove,
                "Delete"
            }
        }
    }
}

#[component]
fn DoctorLetter(
    collection: Option<duet_wire::models::DoctorCollection>,
    base_url: String,
) -> Element {
    let patient = use_context::<PatientSession>();
    let mut exporting = use_signal(|| false);
    let mut export_error = use_signal(String::new);
    let Some(collection) = collection else {
        return rsx! {};
    };
    let Some(preview) = saved_preview(&collection) else {
        return rsx! {};
    };
    let svg = preview.layout.preview_svg();
    let full_text = preview.layout.text().to_owned();
    let publication_url = base_url.clone();
    let generation = collection.note_generation;
    let share = move |_| {
        if exporting() {
            return;
        }
        exporting.set(true);
        let reviewed = collection.clone();
        let url = base_url.clone();
        let session = patient.clone();
        spawn(async move {
            let result = async {
                let api = open_client(url, session.access_token.peek().clone())?;
                let checked = api.checked_doctor_page(reviewed.note_generation).await?;
                if !session.retention.session().is_current(&session.fence) {
                    return Err("Account changed; export cancelled".into());
                }
                if checked != reviewed {
                    return Err(
                        "The saved page changed. Review the new page before exporting.".into(),
                    );
                }
                let preview = saved_preview(&checked).ok_or("No approved page")?;
                if preview.incomplete {
                    return Err("Exact source review required".into());
                }
                let expected = preview
                    .layout
                    .pdf()
                    .map_err(|_| "The full page cannot be exported faithfully")?;
                let bytes = api.doctor_pdf().await.map_err(|e| e.to_string())?;
                if !session.retention.session().is_current(&session.fence) {
                    return Err("Account changed; export cancelled".into());
                }
                if bytes != expected {
                    return Err(
                        "The server PDF differs from the reviewed page. Reload before sharing."
                            .into(),
                    );
                }
                export::present_pdf(&bytes)
            }
            .await;
            if !session.retention.session().is_current(&session.fence) {
                return;
            }
            export_error.set(result.err().unwrap_or_default());
            exporting.set(false);
        });
    };
    rsx! {
        p { class: "card-title", "Saved page preview" }
        {rsx! { publication::PublicationControls { key: "publication-{generation}", base_url: publication_url, generation } }}
        Button { disabled: exporting() || preview.incomplete || !preview.layout.fits(), onclick: share, "Check and share PDF" }
        if !export_error().is_empty() { p { role: "alert", "{export_error}" } }
        // Only our glyph-path renderer supplies this markup; patient text never becomes raw HTML.
        div { class: "doctor-preview", dangerous_inner_html: "{svg}" }
        Disclosure { title: "Read complete page text", detail: true, pre { style: "white-space: pre-wrap", "{full_text}" } }
        if !preview.layout.fits() { p { role: "alert", "The complete page cannot be exported faithfully on one page. Your saved words remain unchanged; shorten the page before exporting." } }
        if preview.incomplete { p { role: "alert", "Some selected information needs exact-source review. Export is blocked until it is checked." } }
    }
}

#[component]
fn PhoneHealthControl(base_url: String, attempt: Signal<u32>) -> Element {
    #[cfg(target_os = "ios")]
    return rsx! { apple_health::AppleHealthControl { base_url, refreshed: attempt } };
    #[cfg(target_os = "android")]
    {
        let mut failed = use_signal(|| false);
        return rsx! { div { class: "source-row",
            span { class: "source-name", "Health Connect" }
            span { class: "source-detail", "You can set phone permissions. Health Connect imports are unavailable in this build." }
            Button { onclick: move |_| failed.set(duet_client::request_health_connect_permission().is_err()), "Allow" }
            if failed() { p { role: "alert", "Could not open Health Connect permissions." } }
        } };
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    rsx! {}
}

#[component]
fn YouTab(
    base_url: String,
    bearer: String,
    alerts: AlertSnapshot,
    on_allow_alerts: EventHandler<()>,
    on_open_settings: EventHandler<()>,
) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let mut doctor_plan_cache = use_signal(|| None::<duet_client::DuetDoctorPlanView>);
    let mut menu = use_signal(|| false);
    let mut editor = use_signal(|| None::<YouEditor>);
    let mut medication = use_signal(String::new);
    let mut note_title = use_signal(String::new);
    let mut note_body = use_signal(String::new);
    let mut medication_id = use_signal(|| None::<uuid::Uuid>);
    let mut note_id = use_signal(|| None::<uuid::Uuid>);
    let mut saving = use_signal(|| false);
    let mut save_failed = use_signal(|| false);
    let mut editing = use_signal(|| None::<EvidenceEdit>);
    let mut evidence_notice = use_signal(|| None::<String>);
    let save_url = base_url.clone();
    let save_bearer = bearer.clone();
    let evidence_url = base_url.clone();
    let evidence_bearer = bearer.clone();
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => {
                    futures_util::future::try_join(api.evidence_library(), async {
                        Ok(api.doctor_plan().await.ok().flatten())
                    })
                    .await
                }
                Err(error) => Err(error),
            }
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok((page, plan))) = resource() {
            cache.set(Some(page));
            doctor_plan_cache.set(plan);
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let (page, doctor_plan) = match current {
        Some(Ok((page, plan))) => (Some(page), plan),
        _ => (cache(), doctor_plan_cache()),
    };
    let root = page.as_ref().map(library_root);
    let plan_section = doctor_plan_section(doctor_plan.as_ref());
    let retry = move |_| attempt.set(attempt() + 1);
    // Saving shares the closure shapes with the other save handlers.
    let save_record = move |_| {
        let Some(kind) = editor() else {
            return;
        };
        if saving() {
            return;
        }
        let (id, name, title, body) = match kind {
            YouEditor::Medication => {
                let name = medication().trim().to_owned();
                if name.is_empty() {
                    return;
                }
                let id = medication_id().unwrap_or_else(uuid::Uuid::new_v4);
                medication_id.set(Some(id));
                (id, Some(name), None, None)
            }
            YouEditor::Note => {
                let title = note_title().trim().to_owned();
                let body = note_body().trim().to_owned();
                if title.is_empty() || body.is_empty() {
                    return;
                }
                let id = note_id().unwrap_or_else(uuid::Uuid::new_v4);
                note_id.set(Some(id));
                (id, None, Some(title), Some(body))
            }
        };
        saving.set(true);
        save_failed.set(false);
        let base_url = save_url.clone();
        let bearer = save_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => match kind {
                    YouEditor::Medication => api.add_medication(id, &name.unwrap()).await,
                    YouEditor::Note => api.add_note(id, &title.unwrap(), &body.unwrap()).await,
                },
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    match kind {
                        YouEditor::Medication => {
                            medication.set(String::new());
                            medication_id.set(None);
                        }
                        YouEditor::Note => {
                            note_title.set(String::new());
                            note_body.set(String::new());
                            note_id.set(None);
                        }
                    }
                    save_failed.set(false);
                    editor.set(None);
                    attempt.set(attempt() + 1);
                }
                Err(_) => save_failed.set(true),
            }
            saving.set(false);
        });
    };
    let correction_url = evidence_url.clone();
    let correction_bearer = evidence_bearer.clone();
    let save_correction = move |_| {
        let Some(current) = editing() else {
            return;
        };
        if current.title.trim().is_empty() || saving() {
            return;
        }
        saving.set(true);
        evidence_notice.set(None);
        let base_url = correction_url.clone();
        let bearer = correction_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => {
                    api.correct_evidence(
                        &current.id,
                        evidence_entity(current.kind),
                        current.title.trim(),
                    )
                    .await
                }
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    evidence_notice.set(Some("Saved the correction.".to_owned()));
                    editing.set(None);
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    evidence_notice.set(Some("This correction couldn’t be saved.".to_owned()));
                }
            }
            saving.set(false);
        });
    };
    rsx! {
        TabHeading { name: "You".to_owned(), loading }
        if let Some(kind) = editor() {
            HealthDraft {
                kind,
                medication,
                note_title,
                note_body,
                saving,
                failed: save_failed,
                on_close: move |_| {
                    save_failed.set(false);
                    editor.set(None);
                },
                on_save: save_record,
            }
        } else if menu() {
            ActionRow {
                Button {
                    onclick: move |_| {
                        menu.set(false);
                        editor.set(Some(YouEditor::Medication));
                    },
                    "New medication"
                }
                Button {
                    onclick: move |_| {
                        menu.set(false);
                        editor.set(Some(YouEditor::Note));
                    },
                    "New note"
                }
                Button {
                    onclick: move |_| menu.set(false),
                    "Close"
                }
            }
        } else {
        PhoneHealthControl { base_url: evidence_url.clone(), attempt }
        if alerts.state.shown() {
            div { class: "source-row",
                span { class: "source-name", "Notifications" }
                span { class: "source-detail", "{alerts.sentence()}" }
                if alerts.state.can_request() {
                    Button {
                        aria_label: "{ALLOW_NOTIFICATIONS}",
                        onclick: move |_| on_allow_alerts.call(()),
                        "Allow"
                    }
                }
            }
            if alerts.state == AlertState::Denied {
                Button {
                    aria_label: "{OPEN_NOTIFICATION_SETTINGS}",
                    onclick: move |_| on_open_settings.call(()),
                    "{OPEN_NOTIFICATION_SETTINGS}"
                }
            }
        }
        if let Some(section) = plan_section {
            article { class: "card voice-card",
                p { class: "card-title", "{section.heading}" }
                p { class: "record-meta", "{section.attribution}" }
                for item in section.items {
                    p { class: "card-text", "{item.title}" }
                    p { class: "record-meta", "{item.body}" }
                }
            }
        }
        if let Some(edit) = editing() {
            input {
                class: "field duet-field",
                r#type: "text",
                aria_label: "Corrected value",
                value: "{edit.title}",
                oninput: move |event| {
                    if let Some(mut current) = editing() {
                        current.title = event.value();
                        editing.set(Some(current));
                    }
                },
            }
            ActionRow {
                Button {
                    variant: ButtonVariant::Primary,
                    aria_label: "Save correction",
                    disabled: saving() || edit.title.trim().is_empty(),
                    onclick: save_correction,
                    "Save correction"
                }
                Button {
                    onclick: move |_| editing.set(None),
                    "Cancel"
                }
            }
        }
        if let Some(notice) = evidence_notice() {
            p { class: "record-meta", role: "status", "{notice}" }
        }
        match root {
            Some(LibraryRoot::Empty) => rsx! { p { class: "empty", "{LIBRARY_EMPTY}" } },
            Some(LibraryRoot::Sections(sections)) => rsx! {
                for section in sections {
                    article { class: if section.measured { "card number-card" } else { "card voice-card" },
                        p { class: "card-title", "{section.name}" }
                        for record in section.records {
                            EvidenceRow {
                                record: record.clone(),
                                base_url: evidence_url.clone(),
                                bearer: evidence_bearer.clone(),
                                editing,
                                notice: evidence_notice,
                                attempt,
                                saving,
                            }
                        }
                    }
                }
            },
            None => rsx! {},
        }
        if failed && page.is_none() {
            div { role: "alert",
                p { class: "empty", "{LIBRARY_UNAVAILABLE}" }
                TryAgain { label: TRY_AGAIN, on_retry: retry }
            }
        }
        records::RecordBrowser { base_url: evidence_url.clone(), bearer: evidence_bearer.clone(), reload: attempt }
        Button {
            onclick: move |_| menu.set(true),
            "Add health information"
        }
        }
    }
}

#[component]
fn DoctorEditor(
    name: Signal<String>,
    birth: Signal<String>,
    note: Signal<String>,
    cover: Signal<String>,
    questions: Signal<Vec<String>>,
    items: Vec<(uuid::Uuid, String)>,
    kept: Signal<Vec<bool>>,
    saving: Signal<bool>,
    failed: Signal<bool>,
    on_save: EventHandler,
) -> Element {
    let ready = doctor_draft_ready(&birth(), &cover(), &questions());
    let can_add = questions().len() < 5;
    rsx! {
        p { class: "status-detail", "Name" }
        input {
            class: "field duet-field",
            r#type: "text",
            aria_label: "Name",
            value: "{name}",
            oninput: move |event| name.set(event.value()),
        }
        p { class: "status-detail", "Date of birth" }
        input {
            class: "field duet-field",
            r#type: "text",
            aria_label: "Date of birth",
            value: "{birth}",
            oninput: move |event| birth.set(event.value()),
        }
        p { class: "status-detail", "What I want to discuss" }
        textarea {
            class: "field duet-field",
            value: "{note}",
            oninput: move |event| note.set(event.value()),
        }
        for index in 0..questions().len() {
            QuestionField { index, questions }
        }
        if can_add {
            Button {
                aria_label: "Add a question",
                onclick: move |_| {
                    let mut fields = questions();
                    fields.push(String::new());
                    questions.set(fields);
                },
                "Add a question"
            }
        }
        p { class: "status-detail", "Cover note" }
        textarea {
            class: "field duet-field",
            value: "{cover}",
            oninput: move |event| cover.set(event.value()),
        }
        for (index, item) in items.iter().enumerate() {
            KeepChoice { index, title: item.1.clone(), kept }
        }
        if failed() {
            p { class: "empty", role: "alert", "This page couldn’t be saved." }
        }
        Button {
            variant: ButtonVariant::Primary,
            disabled: saving() || !ready,
            aria_label: "Save page",
            onclick: move |_| on_save.call(()),
            "Save page"
        }
    }
}

#[component]
fn QuestionField(index: usize, questions: Signal<Vec<String>>) -> Element {
    let value = questions().get(index).cloned().unwrap_or_default();
    rsx! {
        input {
            class: "field duet-field",
            r#type: "text",
            aria_label: "Question",
            value: "{value}",
            oninput: move |event| {
                let mut fields = questions();
                if let Some(field) = fields.get_mut(index) {
                    *field = event.value();
                }
                questions.set(fields);
            },
        }
        Button {
            variant: ButtonVariant::Quiet,
            compact: true,
            danger: true,
            onclick: move |_| {
                let mut fields = questions();
                if index < fields.len() {
                    fields.remove(index);
                    questions.set(fields);
                }
            },
            "Remove"
        }
    }
}

#[component]
fn KeepChoice(index: usize, title: String, kept: Signal<Vec<bool>>) -> Element {
    let included = kept().get(index).copied().unwrap_or(true);
    rsx! {
        Button {
            compact: true,
            onclick: move |_| {
                let mut flags = kept();
                if let Some(flag) = flags.get_mut(index) {
                    *flag = !*flag;
                }
                kept.set(flags);
            },
            if included {
                "{title}"
            } else {
                "{title}, left off"
            }
        }
    }
}

#[component]
fn HealthDraft(
    kind: YouEditor,
    medication: Signal<String>,
    note_title: Signal<String>,
    note_body: Signal<String>,
    saving: Signal<bool>,
    failed: Signal<bool>,
    on_close: EventHandler,
    on_save: EventHandler,
) -> Element {
    let ready = match kind {
        YouEditor::Medication => !medication().trim().is_empty(),
        YouEditor::Note => !note_title().trim().is_empty() && !note_body().trim().is_empty(),
    };
    rsx! {
        Button {
            variant: ButtonVariant::Quiet,
            compact: true,
            onclick: move |_| on_close.call(()),
            "Close"
        }
        match kind {
            YouEditor::Medication => rsx! {
                p { class: "status-detail", "Enter the medication as one descriptive line, including dose when you know it." }
                input {
                    class: "field duet-field",
                    r#type: "text",
                    value: "{medication}",
                    placeholder: "Atorvastatin 20 mg",
                    oninput: move |event| medication.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This medication couldn’t be saved. Nothing was changed." }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Add medication"
                }
            },
            YouEditor::Note => rsx! {
                input {
                    class: "field duet-field",
                    r#type: "text",
                    value: "{note_title}",
                    placeholder: "What is this note about?",
                    oninput: move |event| note_title.set(event.value()),
                }
                textarea {
                    class: "field duet-field",
                    value: "{note_body}",
                    oninput: move |event| note_body.set(event.value()),
                }
                if failed() {
                    p { class: "empty", role: "alert", "This note couldn’t be saved. Nothing was changed." }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    disabled: saving() || !ready,
                    onclick: move |_| on_save.call(()),
                    "Save note"
                }
            },
        }
    }
}
