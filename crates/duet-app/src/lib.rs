//! Duet's four tab roots. SwiftUI remains the shipping iPhone app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

mod account;
mod approval;
mod browser_identity;
mod export;
mod preview;
mod publication;
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
use duet_client::{ConsentState, DuetClient, FrontDoorError};
use duet_health::{HealthSnapshot, NO_READINGS, PhoneReading, PlatformHealth};
use duet_notify::{
    AlertSnapshot, AlertState, PlatformAlerts, insight_alerts_wanted, plan_insight_delivery,
};
use duet_wire::models::ConversationMessage;
use roots::{
    ALLOW_HEALTHKIT, ALLOW_NOTIFICATIONS, AWAITING, CHAT_EMPTY_MESSAGE, CHAT_UNAVAILABLE,
    CHECK_UNAVAILABLE, CHECK_UNAVAILABLE_DETAIL, CONNECTING, ConversationNote, ConversationRoot,
    ConversationSourceChip, DoctorPage, INSIGHT_NOTIFICATIONS, INSIGHT_NOTIFICATIONS_DETAIL,
    JoinDraft, LIBRARY_EMPTY, LIBRARY_UNAVAILABLE, LibraryKind, LibraryRecord, LibraryRoot,
    NOT_SHARING, NoteOpen, OPEN_NOTIFICATION_SETTINGS, REVIEWS_PAUSED, REVIEWS_PAUSED_DETAIL,
    SHARE_WITH_MY_PHYSICIAN, SHARING_ERROR, SHARING_HEADING, SHARING_OFF_DETAIL, SHARING_ON_DETAIL,
    SHARING_SAVED, SHARING_UNAVAILABLE, SHARING_UNCHECKED, STOP_SHARING, SavedItemPage,
    SharedNotice, SharedRoot, SharingRoot, TRY_AGAIN, TodayRoot, closed_send, composer_open,
    composer_text, conversation_root, date_line, doctor_page, doctor_plan_section,
    draft_after_send, greeting, library_root, note_open, opens_doctor, page_with_accepted,
    plan_page, reminder_page, send_id, shared_root, sharing_root, today_root,
};
use ui::{ActionRow, Button, ButtonVariant, TabBar};

const TABS: [&str; 4] = ["Duet", "Today", "You", "Shared"];

#[derive(Clone)]
struct ThreadLoad {
    page: duet_client::ConversationPage,
    routines: Option<Vec<duet_wire::models::SavedRoutine>>,
    follow_ups: Option<Vec<duet_wire::models::FollowUp>>,
}
const DUET_STYLES: &str = include_str!("../assets/duet.css");

pub use roots::{api_base_url, api_bearer};

/// What the front door reports for this session: the consent state Duet's
/// screens turn into the existing practice-sharing controls.
#[derive(Debug, Clone, PartialEq, Eq)]
enum FrontDoorStatus {
    /// The consent read succeeded.
    Loaded(ConsentState),
    /// No practice connection resolves for this session.
    NotJoined,
    /// The front door could not answer.
    Unavailable,
}

async fn load_front_door(base_url: String, bearer: String) -> Result<ConsentState, FrontDoorError> {
    let api = open_client(base_url, bearer).map_err(FrontDoorError::Other)?;
    api.consent().await
}

#[component]
fn PrivateRoot(base_url: String, bearer: String) -> Element {
    let mut tab = use_signal(|| 0usize);
    let mut phone = use_signal(|| PlatformHealth.snapshot());
    let mut alerts = use_signal(|| PlatformAlerts.snapshot());
    let mut insight_alerts = use_signal(|| PlatformAlerts.insights_enabled());
    // Standalone account admission is owned by account::DuetRoot. The front
    // door supplies optional practice-link status; it does not gate the app.
    let mut front_door_attempt = use_signal(|| 0u32);
    let mut joined_session = use_signal(roots::recalled_session);
    let door_url = base_url.clone();
    let door_bearer = bearer.clone();
    let front_door = use_resource(move || {
        let _attempt = front_door_attempt();
        let base_url = door_url.clone();
        let bearer = joined_session().unwrap_or_else(|| door_bearer.clone());
        async move {
            if bearer.is_empty() {
                return Err(FrontDoorError::Unauthorized);
            }
            load_front_door(base_url, bearer).await
        }
    });
    let status = match front_door() {
        Some(Ok(state)) => FrontDoorStatus::Loaded(state),
        Some(Err(FrontDoorError::Unauthorized)) | Some(Err(FrontDoorError::Invalid)) => {
            FrontDoorStatus::NotJoined
        }
        Some(Err(_)) | None => FrontDoorStatus::Unavailable,
    };
    // AccountRoot already verified the standalone account. Consent is optional practice linking.
    let needs_join = false;
    let active_bearer = joined_session().unwrap_or_else(|| bearer.clone());
    rsx! {
        style { "{ui::STYLESHEET}{DUET_STYLES}" }
        main { class: "duet",
            div { class: "duet-page",
                if needs_join {
                    JoinScreen {
                        base_url: base_url.clone(),
                        on_joined: move |session: String| {
                            roots::remember_session(&session);
                            joined_session.set(Some(session));
                            front_door_attempt.set(front_door_attempt() + 1);
                        },
                        on_retry: move |_| front_door_attempt.set(front_door_attempt() + 1),
                        checking: status == FrontDoorStatus::Unavailable,
                    }
                } else {
                div { hidden: tab() != 0, ConversationTab { base_url: base_url.clone(), bearer: active_bearer.clone() } }
                div { hidden: tab() != 1,
                    TodayTab {
                        base_url: base_url.clone(),
                        bearer: active_bearer.clone(),
                        readings: phone().readings.clone(),
                        connected: phone().state.connected(),
                        insight_alerts,
                    }
                }
                div { hidden: tab() != 2,
                    YouTab {
                        base_url: base_url.clone(),
                        bearer: active_bearer.clone(),
                        consent: match &status {
                            FrontDoorStatus::Loaded(state) => Ok(*state),
                            FrontDoorStatus::NotJoined => Err(FrontDoorError::Unauthorized),
                            FrontDoorStatus::Unavailable => Err(FrontDoorError::Unavailable),
                        },
                        phone: phone(),
                        alerts: alerts(),
                        insight_alerts: insight_alerts(),
                        on_allow: move |_| {
                            PlatformHealth.request_access();
                            phone.set(PlatformHealth.snapshot());
                        },
                        on_allow_alerts: move |_| {
                            PlatformAlerts.request_access();
                            alerts.set(PlatformAlerts.snapshot());
                        },
                        on_insight_alerts: move |enabled| {
                            PlatformAlerts.set_insights(enabled);
                            insight_alerts.set(PlatformAlerts.insights_enabled());
                            alerts.set(PlatformAlerts.snapshot());
                        },
                        on_open_settings: move |_| PlatformAlerts.open_settings(),
                    }
                }
                div { hidden: tab() != 3, SharedTab { base_url, bearer: active_bearer } }
                }
            }
            if !needs_join {
                TabBar {
                    labels: TABS.iter().map(|label| (*label).to_owned()).collect(),
                    selected: tab(),
                    on_select: move |index| {
                        tab.set(index);
                        if index == 1 || index == 2 {
                            phone.set(PlatformHealth.snapshot());
                        }
                        if index == 2 {
                            alerts.set(PlatformAlerts.snapshot());
                            insight_alerts.set(PlatformAlerts.insights_enabled());
                        }
                    },
                }
            }
        }
    }
}

/// The join screen: the person enters the email and token their practice's
/// invitation carried, and Duet redeems it at the front door.
#[component]
fn JoinScreen(
    base_url: String,
    on_joined: EventHandler<String>,
    on_retry: EventHandler,
    checking: bool,
) -> Element {
    let mut draft = use_signal(|| JoinDraft {
        email: String::new(),
        token: String::new(),
    });
    let mut email = use_signal(String::new);
    let mut token = use_signal(String::new);
    let mut submitting = use_signal(|| false);
    let mut failure = use_signal(|| None::<FrontDoorError>);
    let join_url = base_url;
    let ready = !email().trim().is_empty() && !token().trim().is_empty();
    let mut submit = move |_| {
        if submitting() {
            return;
        }
        let candidate = JoinDraft {
            email: email().trim().to_owned(),
            token: token().trim().to_owned(),
        };
        if !candidate.ready() {
            return;
        }
        submitting.set(true);
        failure.set(None);
        let base_url = join_url.clone();
        spawn(async move {
            let joined = match open_client(base_url, String::new()) {
                Ok(api) => api.join(&candidate.email, &candidate.token).await,
                Err(error) => Err(FrontDoorError::Other(error)),
            };
            match joined {
                Ok(joined) => {
                    draft.set(JoinDraft {
                        email: String::new(),
                        token: String::new(),
                    });
                    email.set(String::new());
                    token.set(String::new());
                    submitting.set(false);
                    on_joined.call(joined.session);
                }
                Err(error) => {
                    failure.set(Some(error));
                    submitting.set(false);
                }
            }
        });
    };
    rsx! {
        div { class: "duet-page",
            TabHeading { name: "Duet".to_owned(), loading: use_signal(|| false) }
            if checking {
                p { class: "status-detail", "{CONNECTING}" }
                Button {
                    compact: true,
                    onclick: move |_| on_retry.call(()),
                    "{TRY_AGAIN}"
                }
            }
            p { class: "status-title", "{roots::JOIN_HEADING}" }
            p { class: "status-detail", "{roots::JOIN_DETAIL}" }
            form {
                class: "duet-composer",
                onsubmit: move |event| {
                    event.prevent_default();
                    submit(());
                },
                p { class: "status-detail", "{roots::JOIN_EMAIL_LABEL}" }
                input {
                    class: "duet-field",
                    r#type: "email",
                    name: "email",
                    value: "{email}",
                    oninput: move |event| email.set(event.value()),
                }
                p { class: "status-detail", "{roots::JOIN_TOKEN_LABEL}" }
                input {
                    class: "duet-field",
                    r#type: "text",
                    name: "token",
                    value: "{token}",
                    oninput: move |event| token.set(event.value()),
                }
                if let Some(error) = failure() {
                    p { class: "empty", role: "alert", "{roots::join_failure_sentence(&error)}" }
                    if error == FrontDoorError::Unavailable {
                        Button {
                            compact: true,
                            submit: true,
                            "{roots::JOIN_RETRY}"
                        }
                    }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    submit: true,
                    disabled: submitting() || !ready,
                    "{roots::JOIN_BUTTON}"
                }
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

#[component]
fn TryAgain(on_retry: EventHandler) -> Element {
    rsx! {
        Button {
            compact: true,
            onclick: move |_| on_retry.call(()),
            "{TRY_AGAIN}"
        }
    }
}

fn open_client(base_url: String, bearer: String) -> Result<DuetClient, String> {
    let bearer = if let Some(session) = try_consume_context::<PatientSession>() {
        if !session.retention.session().is_current(&session.fence) {
            return Err("Account session ended".into());
        }
        session.access_token.peek().clone()
    } else {
        bearer
    };
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
struct OpenedSource {
    title: String,
    text: Option<String>,
    failed: bool,
}

#[derive(Clone)]
struct EvidenceEdit {
    id: String,
    kind: LibraryKind,
    title: String,
}

#[derive(Clone)]
enum ShownPage {
    Doctor(DoctorPage),
    Item(SavedItemPage),
}

impl ShownPage {
    fn heading(&self) -> String {
        match self {
            ShownPage::Doctor(_) => "For my doctor".to_owned(),
            ShownPage::Item(item) => item.heading.clone(),
        }
    }
}

async fn load_today(
    base_url: String,
    bearer: String,
) -> Result<
    (
        duet_client::IntelligenceSettings,
        duet_client::InsightCyclePage,
    ),
    String,
> {
    let api = open_client(base_url, bearer)?;
    let settings = api.intelligence_settings().await?;
    let cycles = api.list_insight_cycles().await?;
    Ok((settings, cycles))
}

fn schedule_loaded_insights(page: &duet_client::InsightCyclePage) {
    let state = PlatformAlerts.snapshot().state;
    if state == AlertState::Allowed {
        PlatformAlerts.register_remote();
    }
    if !insight_alerts_wanted(PlatformAlerts.insights_enabled(), state) {
        PlatformAlerts.cancel_insight();
        return;
    }
    let cycles: Vec<(uuid::Uuid, bool)> = page
        .items
        .iter()
        .map(|cycle| {
            (
                cycle.id,
                cycle
                    .insights
                    .as_ref()
                    .is_some_and(|insights| !insights.is_empty()),
            )
        })
        .collect();
    let Some(delivery) = plan_insight_delivery(
        chrono::Local::now(),
        &cycles,
        PlatformAlerts.last_planned_cycle(),
    ) else {
        return;
    };
    PlatformAlerts.submit_insight(&delivery);
}

#[component]
fn ConversationTab(base_url: String, bearer: String) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let mut draft = use_signal(String::new);
    let mut pending_id = use_signal(|| None::<(uuid::Uuid, String)>);
    let mut sending = use_signal(|| false);
    let mut send_failed = use_signal(|| false);
    let mut accepted = use_signal(|| None::<ConversationMessage>);
    let mut routines = use_signal(Vec::new);
    let mut follow_ups = use_signal(Vec::new);
    let mut doctor = use_signal(|| None);
    let mut open_item = use_signal(|| None::<NoteOpen>);
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
            open_item.set(Some(NoteOpen::Doctor));
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
    let mut editing = use_signal(|| false);
    let mut plan_title = use_signal(String::new);
    let mut plan_body = use_signal(String::new);
    let mut confirm_remove = use_signal(|| false);
    let mut removed_plan = use_signal(|| false);
    let mut plan_save_failed = use_signal(|| false);
    let mut plan_remove_failed = use_signal(|| false);
    let mut follow_failed = use_signal(|| false);
    let mut changing = use_signal(|| false);
    let mut opened_source = use_signal(|| None::<OpenedSource>);
    let export_url = base_url.clone();
    let send_url = base_url.clone();
    let send_bearer = bearer.clone();
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
    let mut provisional = use_signal(duet_client::streaming::Provisional::default);
    let stream_url = base_url.clone();
    let stream_bearer = bearer.clone();
    let stream_owner = patient.retention.clone();
    let stream_fence = patient.fence.clone();
    use_future(move || {
        let url = stream_url.clone();
        let bearer = stream_bearer.clone();
        let owner = stream_owner.clone();
        let fence = stream_fence.clone();
        async move {
            let Ok(api) = open_client(url, bearer) else {
                return;
            };
            loop {
                if !owner.session().is_current(&fence) {
                    provisional.set(Default::default());
                    break;
                }
                let command = cache
                    .peek()
                    .as_ref()
                    .and_then(|page: &duet_client::ConversationPage| page.active_run.as_ref())
                    .and_then(|run| run.as_ref())
                    .map(|run| run.id);
                if let Some(command) = command {
                    let mut next = provisional.peek().clone();
                    if next.command != Some(command) {
                        next = duet_client::streaming::Provisional::new(command);
                    }
                    let prior = next.after;
                    match api.conversation_events(command, next.after).await {
                        Ok(events) if owner.session().is_current(&fence) => {
                            if next.apply(command, &events).is_err() {
                                next.clear();
                            }
                            if next.stopped && next.after != prior {
                                attempt.set(attempt() + 1);
                            }
                            provisional.set(next);
                        }
                        _ => {
                            next.clear();
                            provisional.set(next);
                        }
                    }
                } else if provisional.peek().command.is_some() {
                    provisional.set(Default::default());
                }
                futures_timer::Delay::new(Duration::from_millis(500)).await;
            }
        }
    });
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            let api = open_client(base_url, bearer)?;
            let page = api.list_messages().await?;
            let routines = api.list_saved_routines().await.ok().map(|list| list.items);
            let follow_ups = api.list_follow_ups().await.ok().map(|page| page.items);
            Ok::<_, String>(ThreadLoad {
                page,
                routines,
                follow_ups,
            })
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok(loaded)) = resource() {
            if let Some(items) = loaded.routines.clone() {
                routines.set(items);
            }
            if let Some(items) = loaded.follow_ups.clone() {
                follow_ups.set(items);
            }
            let page = match accepted() {
                Some(message) => {
                    let merged = page_with_accepted(loaded.page.clone(), message.clone());
                    if loaded
                        .page
                        .messages
                        .iter()
                        .any(|item| item.id == message.id)
                    {
                        accepted.set(None);
                    }
                    merged
                }
                None => loaded.page,
            };
            cache.set(Some(page));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let page = if accepted().is_some() {
        cache()
    } else {
        match current {
            Some(Ok(loaded)) => Some(loaded.page),
            _ => cache(),
        }
    };
    let root = page
        .as_ref()
        .map(|page| conversation_root(page, &routines(), &follow_ups(), doctor().as_ref()));
    let retry = move |_| attempt.set(attempt() + 1);
    let shown = match open_item() {
        Some(NoteOpen::Doctor) => doctor().as_ref().map(doctor_page).map(ShownPage::Doctor),
        Some(NoteOpen::Routine(_)) if removed_plan() => Some(ShownPage::Item(SavedItemPage {
            heading: "Plan".to_owned(),
            title: None,
            detail: Some("This saved plan was removed.".to_owned()),
        })),
        Some(NoteOpen::Routine(id)) => routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(plan_page)
            .map(ShownPage::Item),
        Some(NoteOpen::FollowUp(id)) => follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(reminder_page)
            .map(ShownPage::Item),
        None => None,
    };
    let can_compose = page.as_ref().is_some_and(composer_open) && shown.is_none();
    let closed = page.as_ref().and_then(closed_send);
    let page_url = send_url.clone();
    let page_bearer = send_bearer.clone();
    let mut submit = move |text: String| {
        if sending() {
            return;
        }
        if text.trim().is_empty() {
            return;
        }
        draft.set(text.clone());
        let pending = pending_id();
        let id = send_id(
            pending
                .as_ref()
                .map(|(id, pending_text)| (*id, pending_text.as_str())),
            &text,
            uuid::Uuid::new_v4(),
        );
        pending_id.set(Some((id, text.clone())));
        sending.set(true);
        send_failed.set(false);
        let base_url = send_url.clone();
        let bearer = send_bearer.clone();
        let shown = page.clone();
        spawn(async move {
            let timezone = match iana_time_zone::get_timezone() {
                Ok(timezone) => timezone,
                Err(_) => {
                    send_failed.set(true);
                    sending.set(false);
                    return;
                }
            };
            let api = match open_client(base_url, bearer) {
                Ok(api) => api,
                Err(_) => {
                    send_failed.set(true);
                    sending.set(false);
                    return;
                }
            };
            match api.send_message(id, &text, &timezone).await {
                Ok(admission) => {
                    let message = *admission.message;
                    accepted.set(Some(message.clone()));
                    if let Some(base) = shown {
                        cache.set(Some(page_with_accepted(base, message)));
                    }
                    draft.set(draft_after_send(&draft(), true));
                    pending_id.set(None);
                    send_failed.set(false);
                    attempt.set(attempt() + 1);
                }
                Err(_) => {
                    draft.set(draft_after_send(&draft(), false));
                    send_failed.set(true);
                }
            }
            sending.set(false);
        });
    };
    let plan_url = page_url.clone();
    let plan_bearer = page_bearer.clone();
    let remove_url = page_url.clone();
    let remove_bearer = page_bearer.clone();
    let pause_url = page_url.clone();
    let pause_bearer = page_bearer.clone();
    let resume_url = page_url.clone();
    let resume_bearer = page_bearer.clone();
    let fresh_url = page_url.clone();
    let fresh_bearer = page_bearer.clone();
    let source_url = page_url.clone();
    let source_bearer = page_bearer.clone();
    let save_plan = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::Routine(id)) = open_item() else {
            return;
        };
        let title = plan_title().trim().to_owned();
        let body = plan_body().trim().to_owned();
        if title.is_empty() || body.is_empty() {
            return;
        }
        let Some(revision) = routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(|routine| routine.revision)
        else {
            return;
        };
        changing.set(true);
        plan_save_failed.set(false);
        let base_url = plan_url.clone();
        let bearer = plan_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.update_saved_routine(id, revision, &title, &body).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(routine) => {
                    let mut items = routines();
                    if let Some(item) = items.iter_mut().find(|item| item.id == routine.id) {
                        *item = routine;
                    }
                    routines.set(items);
                    editing.set(false);
                    plan_save_failed.set(false);
                }
                Err(_) => plan_save_failed.set(true),
            }
            changing.set(false);
        });
    };
    let remove_plan = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::Routine(id)) = open_item() else {
            return;
        };
        let Some(revision) = routines()
            .iter()
            .find(|routine| routine.id == id)
            .map(|routine| routine.revision)
        else {
            return;
        };
        changing.set(true);
        plan_remove_failed.set(false);
        let base_url = remove_url.clone();
        let bearer = remove_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.delete_saved_routine(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(()) => {
                    let mut items = routines();
                    items.retain(|item| item.id != id);
                    routines.set(items);
                    editing.set(false);
                    confirm_remove.set(false);
                    removed_plan.set(true);
                    plan_remove_failed.set(false);
                }
                Err(_) => plan_remove_failed.set(true),
            }
            changing.set(false);
        });
    };
    let pause_reminder = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::FollowUp(id)) = open_item() else {
            return;
        };
        let Some(revision) = follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(|follow_up| follow_up.revision)
        else {
            return;
        };
        changing.set(true);
        follow_failed.set(false);
        let base_url = pause_url.clone();
        let bearer = pause_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.pause_follow_up(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(follow_up) => {
                    let mut items = follow_ups();
                    if let Some(item) = items.iter_mut().find(|item| item.id == follow_up.id) {
                        *item = follow_up;
                    }
                    follow_ups.set(items);
                    follow_failed.set(false);
                }
                Err(_) => follow_failed.set(true),
            }
            changing.set(false);
        });
    };
    let resume_reminder = move |_| {
        if changing() {
            return;
        }
        let Some(NoteOpen::FollowUp(id)) = open_item() else {
            return;
        };
        let Some(revision) = follow_ups()
            .iter()
            .find(|follow_up| follow_up.id == id)
            .map(|follow_up| follow_up.revision)
        else {
            return;
        };
        changing.set(true);
        follow_failed.set(false);
        let base_url = resume_url.clone();
        let bearer = resume_bearer.clone();
        spawn(async move {
            let saved = match open_client(base_url, bearer) {
                Ok(api) => api.resume_follow_up(id, revision).await,
                Err(_) => Err(String::new()),
            };
            match saved {
                Ok(follow_up) => {
                    let mut items = follow_ups();
                    if let Some(item) = items.iter_mut().find(|item| item.id == follow_up.id) {
                        *item = follow_up;
                    }
                    follow_ups.set(items);
                    follow_failed.set(false);
                }
                Err(_) => follow_failed.set(true),
            }
            changing.set(false);
        });
    };
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
        resolution::OperationRecovery { base_url: doctor_source_url.clone(), on_released: move |_| { attempt.set(attempt() + 1); open_item.set(None); } }
        selection::DeviceSources { base_url: doctor_source_url.clone(), blocked: source_blocked, on_selected: move |collection| { doctor.set(Some(collection)); open_item.set(Some(NoteOpen::Doctor)); }, on_recheck: move |_| attempt.set(attempt() + 1) }
        selection::DoctorSources { base_url: doctor_source_url, blocked: source_blocked, on_selected: move |collection| { doctor.set(Some(collection)); open_item.set(Some(NoteOpen::Doctor)); } }
        if let Some(Err(error)) = workspace() { p { role: "alert", "For my doctor: {error}" } }
        if doctor().is_some() && open_item().is_none() { Button { onclick: move |_| open_item.set(Some(NoteOpen::Doctor)), "Open For my doctor" } }
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
            TabHeading { name: shown.heading(), loading }
            Button {
                variant: ButtonVariant::Quiet,
                compact: true,
                aria_label: "Close page",
                onclick: move |_| {
                    open_item.set(None);
                    editing.set(false);
                    doctor_editing.set(false);
                    confirm_fresh.set(false);
                    confirm_remove.set(false);
                    removed_plan.set(false);
                    plan_save_failed.set(false);
                    plan_remove_failed.set(false);
                    follow_failed.set(false);
                    page_failed.set(false);
                    fresh_failed.set(false);
                },
                "Close"
            }
            match shown {
                ShownPage::Doctor(DoctorPage::Lines(entries)) if !doctor_editing() => rsx! {
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
                ShownPage::Doctor(DoctorPage::Empty) if !doctor_editing() => rsx! {
                    p { class: "empty", "{LIBRARY_EMPTY}" }
                    DoctorLetter { collection: doctor(), base_url: export_url.clone() }
                },
                ShownPage::Doctor(_) => rsx! {},
                ShownPage::Item(item) if !editing() => rsx! {
                    div { class: "duet-thread",
                        if let Some(title) = item.title {
                            p { class: "card-text", "{title}" }
                        }
                        if let Some(detail) = item.detail {
                            p { class: "record-meta", "{detail}" }
                        }
                        if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id && follow_up.state == duet_wire::models::FollowUpState::Paused)) {
                            p { class: "record-meta", "Paused" }
                        }
                    }
                },
                ShownPage::Item(_) => rsx! {},
            }
            if editing() {
                PlanEditor {
                    title: plan_title,
                    body: plan_body,
                    saving: changing,
                    failed: plan_save_failed,
                    on_save: save_plan,
                }
            }
            if !editing() && !removed_plan() && !confirm_remove() && matches!(open_item(), Some(NoteOpen::Routine(_))) {
                ActionRow {
                    Button {
                        disabled: changing(),
                        onclick: move |_| {
                            let Some(NoteOpen::Routine(id)) = open_item() else {
                                return;
                            };
                            let Some(routine) = routines().iter().find(|routine| routine.id == id).cloned() else {
                                return;
                            };
                            plan_title.set(routine.title);
                            plan_body.set(routine.body);
                            plan_save_failed.set(false);
                            editing.set(true);
                        },
                        "Edit plan"
                    }
                    Button {
                        danger: true,
                        disabled: changing(),
                        onclick: move |_| {
                            plan_remove_failed.set(false);
                            confirm_remove.set(true);
                        },
                        "Remove plan"
                    }
                }
                if plan_remove_failed() {
                    p { class: "empty", role: "alert", "The plan couldn’t be removed. Review the current version and try again." }
                }
            }
            if confirm_remove() {
                p { class: "status-title", "Remove this plan?" }
                p { class: "status-detail", "This removes the saved plan and discards your unsaved edits. Your conversation stays in Duet." }
                ActionRow {
                    Button {
                        variant: ButtonVariant::Primary,
                        danger: true,
                        disabled: changing(),
                        onclick: remove_plan,
                        "Remove"
                    }
                    Button {
                        disabled: changing(),
                        onclick: move |_| confirm_remove.set(false),
                        "Cancel"
                    }
                }
            }
            if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id)) {
                if follow_failed() {
                    p { class: "empty", role: "alert", "Duet couldn’t change this scheduled item. Its last saved state is still here." }
                }
                if matches!(open_item(), Some(NoteOpen::FollowUp(id)) if follow_ups().iter().any(|follow_up| follow_up.id == id && follow_up.state == duet_wire::models::FollowUpState::Paused)) {
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: changing(),
                        onclick: resume_reminder,
                        "Resume reminder"
                    }
                } else {
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: changing(),
                        onclick: pause_reminder,
                        "Pause reminder"
                    }
                }
            }
            if matches!(open_item(), Some(NoteOpen::Doctor)) && doctor_editing() && !draft_uncertain() {
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
            if matches!(open_item(), Some(NoteOpen::Doctor)) && !doctor_editing() && confirm_fresh() {
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
            if matches!(open_item(), Some(NoteOpen::Doctor)) && !doctor_editing() && !confirm_fresh() {
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
        TabHeading { name: "Duet".to_owned(), loading }
        match root {
            None if !failed => rsx! {},
            None => rsx! {
                div { role: "alert",
                    p { class: "empty", "{CHAT_UNAVAILABLE}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(ConversationRoot::Empty { title, message }) => rsx! {
                div { class: "duet-thread",
                    p { class: "empty-title", "{title}" }
                    p { class: "empty", "{message}" }
                    if failed {
                        TryAgain { on_retry: retry }
                    }
                }
            },
            Some(ConversationRoot::Messages(lines)) => rsx! {
                if let Some(open) = opened_source() {
                    article { class: "voice-card",
                        p { class: "card-title", "{open.title}" }
                        if let Some(text) = open.text {
                            p { class: "card-text", "{text}" }
                        }
                        if open.failed {
                            p { class: "empty", role: "alert", "This source couldn’t be opened." }
                        }
                        Button {
                            aria_label: "Close source",
                            onclick: move |_| opened_source.set(None),
                            "Close"
                        }
                    }
                }
                div { class: "duet-thread",
                    for line in lines {
                        article { class: if line.from_duet { "voice-card" } else { "user-bubble" },
                            p { class: "card-title", "{line.role}" }
                            p { class: "card-text", "{line.text}" }
                            for source in line.sources {
                                SourceButton {
                                    chip: source.clone(),
                                    base_url: source_url.clone(),
                                    bearer: source_bearer.clone(),
                                    opened: opened_source,
                                }
                            }
                            for note in line.notes {
                                ThreadNote {
                                    note: note.clone(),
                                    open: note_open(&note, doctor().is_some(), &routines(), &follow_ups()),
                                    on_open: move |which| open_item.set(Some(which)),
                                }
                            }
                        }
                    }
                    if provisional().stopped {
                        p { class: "status-detail", "Draft ended. Checking the saved response; unfinished work remains available for recovery." }
                    }
                    if !provisional().text.is_empty() {
                        article { class: "voice-card", aria_label: "Provisional response",
                            p { class: "card-title", "Luna — drafting, not yet saved" }
                            p { class: "card-text", "{provisional().text}" }
                        }
                    }
                    if let Some(closed) = closed {
                        p { class: "empty-title", "{closed.title}" }
                        p { class: "empty", "{closed.message}" }
                    }
                    if failed {
                        div { role: "alert",
                            p { class: "empty", "{CHAT_UNAVAILABLE}" }
                            TryAgain { on_retry: retry }
                        }
                    }
                }
            },
        }
        if can_compose {
            div { class: "duet-composer-wrap",
                if send_failed() {
                    p { class: "empty", role: "alert", "{CHAT_UNAVAILABLE}" }
                }
                form {
                    class: "duet-composer",
                    onsubmit: move |event| {
                        event.prevent_default();
                        let typed = event.get_first("draft").and_then(|value| match value {
                            dioxus::events::FormValue::Text(text) => Some(text),
                            dioxus::events::FormValue::File(_) => None,
                        });
                        submit(composer_text(typed.as_deref(), &draft()));
                    },
                    textarea {
                        class: "duet-composer-field",
                        name: "draft",
                        aria_label: "{CHAT_EMPTY_MESSAGE}",
                        rows: "1",
                        value: "{draft}",
                        oninput: move |event| draft.set(event.value()),
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        compact: true,
                        icon_only: true,
                        class: "duet-send",
                        submit: true,
                        disabled: sending(),
                        aria_label: "Send",
                        "↑"
                    }
                }
            }
        }
        }
    }
}

#[component]
fn ThreadNote(
    note: ConversationNote,
    open: Option<NoteOpen>,
    on_open: EventHandler<NoteOpen>,
) -> Element {
    if let Some(open) = open {
        let label = if opens_doctor(&note) {
            "Open For my doctor"
        } else {
            "Open saved item"
        };
        let body = note_body(note);
        rsx! {
            button {
                class: "thread-open",
                r#type: "button",
                aria_label: "{label}",
                onclick: move |_| on_open.call(open),
                {body}
            }
        }
    } else {
        note_body(note)
    }
}

#[component]
fn SourceButton(
    chip: ConversationSourceChip,
    base_url: String,
    bearer: String,
    opened: Signal<Option<OpenedSource>>,
) -> Element {
    let title = chip.title.clone();
    let label = format!("Open source {title}");
    let open = move |_| load_source(opened, base_url.clone(), bearer.clone(), chip.clone());
    rsx! {
        Button {
            variant: ButtonVariant::Quiet,
            compact: true,
            aria_label: "{label}",
            onclick: open,
            "{title}"
        }
    }
}

fn load_source(
    mut opened: Signal<Option<OpenedSource>>,
    base_url: String,
    bearer: String,
    chip: ConversationSourceChip,
) {
    let title = chip.title.clone();
    opened.set(Some(OpenedSource {
        title: title.clone(),
        text: None,
        failed: false,
    }));
    spawn(async move {
        let loaded = match open_client(base_url, bearer) {
            Ok(api) => api.message_source(chip.message_id, &chip.source_id).await,
            Err(_) => Err(String::new()),
        };
        match loaded {
            Ok(detail) => opened.set(Some(OpenedSource {
                title,
                text: Some(if detail.superseded.flatten() == Some(true) {
                    format!("This is the cited version. A newer version of this record is available.\n\n{}", detail.text)
                } else {
                    detail.text
                }),
                failed: false,
            })),
            Err(_) => opened.set(Some(OpenedSource {
                title,
                text: None,
                failed: true,
            })),
        }
    });
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
        details { summary { "Read complete page text" } pre { style: "white-space: pre-wrap", "{full_text}" } }
        if !preview.layout.fits() { p { role: "alert", "The complete page cannot be exported faithfully on one page. Your saved words remain unchanged; shorten the page before exporting." } }
        if preview.incomplete { p { role: "alert", "Some selected information needs exact-source review. Export is blocked until it is checked." } }
    }
}

fn note_body(note: ConversationNote) -> Element {
    match note {
        ConversationNote::Card(card) => rsx! {
            div { class: "thread-card",
                p { class: "card-title", "{card.receipt}" }
                if let Some(title) = card.title {
                    p { class: "card-text", "{title}" }
                }
                if let Some(detail) = card.detail {
                    p { class: "record-meta", "{detail}" }
                }
                if let Some(schedule) = card.schedule {
                    p { class: "record-meta", "{schedule}" }
                }
                for entry in card.entries {
                    p { class: "card-text", "{entry.title}" }
                    if let Some(detail) = entry.detail {
                        p { class: "record-meta", "{detail}" }
                    }
                }
            }
        },
        ConversationNote::Receipt(sentence) => rsx! {
            p { class: "record-meta thread-card", "{sentence}" }
        },
    }
}

#[component]
fn TodayTab(
    base_url: String,
    bearer: String,
    readings: Vec<PhoneReading>,
    connected: bool,
    insight_alerts: Signal<bool>,
) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        load_today(base_url, bearer)
    });
    use_effect(move || {
        let _preference = insight_alerts();
        loading.set(resource().is_none());
        if let Some(Ok(value)) = resource() {
            schedule_loaded_insights(&value.1);
            cache.set(Some(value));
        }
    });
    let current = resource();
    let failed = matches!(current, Some(Err(_)));
    let ready = match current {
        Some(Ok(value)) => Some(value),
        _ => cache(),
    };
    let root = ready
        .as_ref()
        .map(|(settings, cycles)| today_root(settings, cycles));
    let now = chrono::Local::now();
    let date = date_line(now);
    let hello = greeting(now.format("%H").to_string().parse().unwrap_or(0));
    let retry = move |_| attempt.set(attempt() + 1);
    let show_unavailable = failed && !matches!(root, Some(TodayRoot::Insights(_)));
    rsx! {
        TabHeading { name: "Today".to_owned(), loading }
        p { class: "duet-date", "{date}" }
        p { class: "duet-greeting", "{hello}" }
        PhoneReadings { readings, connected }
        match root {
            Some(TodayRoot::Quiet) => rsx! {},
            Some(TodayRoot::Paused) => rsx! {
                p { class: "status-title", "{REVIEWS_PAUSED}" }
                p { class: "status-detail", "{REVIEWS_PAUSED_DETAIL}" }
            },
            Some(TodayRoot::Insights(cards)) => rsx! {
                for card in cards {
                    article { class: "voice-card",
                        p { class: "card-title", "{card.title}" }
                        p { class: "card-text", "{card.sentence}" }
                    }
                }
            },
            None => rsx! {},
        }
        if show_unavailable {
            div { role: "alert",
                p { class: "status-title", "{CHECK_UNAVAILABLE}" }
                p { class: "status-detail", "{CHECK_UNAVAILABLE_DETAIL}" }
                TryAgain { on_retry: retry }
            }
        }
    }
}

#[component]
fn YouTab(
    base_url: String,
    bearer: String,
    consent: Result<ConsentState, FrontDoorError>,
    phone: HealthSnapshot,
    alerts: AlertSnapshot,
    insight_alerts: bool,
    on_allow: EventHandler<()>,
    on_allow_alerts: EventHandler<()>,
    on_insight_alerts: EventHandler<bool>,
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
    let mut changing_sharing = use_signal(|| false);
    let mut sharing_failed = use_signal(|| false);
    let mut editing = use_signal(|| None::<EvidenceEdit>);
    let mut evidence_notice = use_signal(|| None::<String>);
    let share_url = base_url.clone();
    let share_bearer = bearer.clone();
    let save_url = base_url.clone();
    let save_bearer = bearer.clone();
    let evidence_url = base_url.clone();
    let evidence_bearer = bearer.clone();
    let sharing = sharing_root(consent.clone());
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => {
                    let library = api.evidence_library().await?;
                    let doctor_plan = api.doctor_plan().await.ok().flatten();
                    Ok((library, doctor_plan))
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
        div { class: "source-row",
            span { class: "source-name", "{SHARING_HEADING}" }
            span { class: "source-detail", match sharing {
                SharingRoot::On => SHARING_ON_DETAIL,
                SharingRoot::Off | SharingRoot::NotJoined => SHARING_OFF_DETAIL,
                SharingRoot::Unknown => SHARING_UNAVAILABLE,
            } }
            if sharing == SharingRoot::On || sharing == SharingRoot::Off {
                input {
                    r#type: "checkbox",
                    checked: sharing == SharingRoot::On,
                    aria_label: "{SHARING_HEADING}",
                    disabled: changing_sharing(),
                    onchange: move |event| {
                        let on = event.checked();
                        if changing_sharing() {
                            return;
                        }
                        changing_sharing.set(true);
                        sharing_failed.set(false);
                        let base_url = share_url.clone();
                        let bearer = share_bearer.clone();
                        spawn(async move {
                            let done = match open_client(base_url, bearer) {
                                Ok(api) => {
                                    if on {
                                        api.grant_consent().await
                                    } else {
                                        api.withdraw_consent().await
                                    }
                                }
                                Err(error) => Err(FrontDoorError::Other(error)),
                            };
                            match done {
                                Ok(()) => sharing_failed.set(false),
                                Err(_) => sharing_failed.set(true),
                            }
                            changing_sharing.set(false);
                        });
                    },
                }
            }
        }
        if sharing_failed() {
            p { class: "empty", role: "alert", "{SHARING_ERROR}" }
        }
        div { class: "source-row",
            span { class: "source-name", "Sources" }
            span { class: "source-detail", "{phone.sentence()}" }
            if phone.state.can_request() {
                Button {
                    aria_label: "{ALLOW_HEALTHKIT}",
                    onclick: move |_| on_allow.call(()),
                    "Allow"
                }
            }
        }
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
            div { class: "source-row",
                span { class: "source-name", "{INSIGHT_NOTIFICATIONS}" }
                span { class: "source-detail", "{INSIGHT_NOTIFICATIONS_DETAIL}" }
                input {
                    r#type: "checkbox",
                    checked: insight_alerts,
                    aria_label: "{INSIGHT_NOTIFICATIONS}",
                    onchange: move |event| on_insight_alerts.call(event.checked()),
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
        PhoneReadings { readings: phone.readings.clone(), connected: phone.state.connected() }
        if let Some(section) = plan_section {
            article { class: "voice-card",
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
                class: "duet-field",
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
                    article { class: if section.measured { "number-card" } else { "voice-card" },
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
                TryAgain { on_retry: retry }
            }
        }
        Button {
            onclick: move |_| menu.set(true),
            "Add health information"
        }
        }
    }
}

#[component]
fn PlanEditor(
    title: Signal<String>,
    body: Signal<String>,
    saving: Signal<bool>,
    failed: Signal<bool>,
    on_save: EventHandler,
) -> Element {
    let ready = !title().trim().is_empty() && !body().trim().is_empty();
    rsx! {
        input {
            class: "duet-field",
            r#type: "text",
            value: "{title}",
            oninput: move |event| title.set(event.value()),
        }
        textarea {
            class: "duet-field",
            value: "{body}",
            oninput: move |event| body.set(event.value()),
        }
        if failed() {
            p { class: "empty", role: "alert", "Your changes haven’t been confirmed. Your words are kept; refresh to check the saved version." }
        }
        Button {
            variant: ButtonVariant::Primary,
            disabled: saving() || !ready,
            onclick: move |_| on_save.call(()),
            "Save changes"
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
            class: "duet-field",
            r#type: "text",
            aria_label: "Name",
            value: "{name}",
            oninput: move |event| name.set(event.value()),
        }
        p { class: "status-detail", "Date of birth" }
        input {
            class: "duet-field",
            r#type: "text",
            aria_label: "Date of birth",
            value: "{birth}",
            oninput: move |event| birth.set(event.value()),
        }
        p { class: "status-detail", "What I want to discuss" }
        textarea {
            class: "duet-field",
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
            class: "duet-field",
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
            class: "duet-field",
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
                    class: "duet-field",
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
                    class: "duet-field",
                    r#type: "text",
                    value: "{note_title}",
                    placeholder: "What is this note about?",
                    oninput: move |event| note_title.set(event.value()),
                }
                textarea {
                    class: "duet-field",
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

#[component]
fn PhoneReadings(readings: Vec<PhoneReading>, connected: bool) -> Element {
    rsx! {
        if connected && readings.is_empty() {
            p { class: "empty", "{NO_READINGS}" }
        }
        for reading in readings {
            article { class: "number-card",
                p { class: "card-title", "{reading.name}" }
                p { class: "card-text", "{reading.value} {reading.unit}" }
                if !reading.source.is_empty() {
                    p { class: "record-meta", "{reading.source}" }
                }
            }
        }
    }
}

#[component]
fn SharedTab(base_url: String, bearer: String) -> Element {
    let mut attempt = use_signal(|| 0u32);
    let mut loading = use_signal(|| true);
    let mut cache = use_signal(|| None);
    let mut physician_cache = use_signal(|| None::<duet_client::PhysicianSharingStatus>);
    let action_url = base_url.clone();
    let action_bearer = bearer.clone();
    let resource = use_resource(move || {
        let _attempt = attempt();
        let base_url = base_url.clone();
        let bearer = bearer.clone();
        async move {
            match open_client(base_url, bearer) {
                Ok(api) => {
                    let overview = api.sharing_overview().await?;
                    let physician = api.physician_sharing_status().await.ok();
                    Ok((overview, physician))
                }
                Err(error) => Err(error),
            }
        }
    });
    use_effect(move || {
        loading.set(resource().is_none());
        if let Some(Ok((overview, physician))) = resource() {
            cache.set(Some(shared_root(&[], Ok(&overview))));
            physician_cache.set(physician);
        }
    });
    let current = resource();
    let (view, physician_status) = match &current {
        Some(Ok((overview, physician))) => (shared_root(&[], Ok(overview)), physician.clone()),
        Some(Err(_)) => (
            shared_root(
                &cache().map(|root| root.people).unwrap_or_default(),
                Err(()),
            ),
            physician_cache(),
        ),
        None => (
            cache().unwrap_or(SharedRoot {
                people: Vec::new(),
                notice: None,
            }),
            physician_cache(),
        ),
    };
    let people = view.people.clone();
    let notice = view.notice.clone();
    let retry = move |_| attempt.set(attempt() + 1);
    let stop_url = action_url.clone();
    let stop_bearer = action_bearer.clone();
    let stop_sharing = move |_| {
        let base_url = stop_url.clone();
        let bearer = stop_bearer.clone();
        spawn(async move {
            if let Ok(api) = open_client(base_url, bearer) {
                let _ = api.stop_physician_sharing().await;
                attempt.set(attempt() + 1);
            }
        });
    };
    let grant_sharing = move |_| {
        let base_url = action_url.clone();
        let bearer = action_bearer.clone();
        spawn(async move {
            if let Ok(api) = open_client(base_url, bearer) {
                let _ = api.grant_physician_consent("v1").await;
                attempt.set(attempt() + 1);
            }
        });
    };
    rsx! {
        TabHeading { name: "Shared".to_owned(), loading }
        if let Some(status) = physician_status.filter(|status| status.linked) {
            article { class: "voice-card",
                p { class: "card-title", "{status.practice_name.as_deref().unwrap_or(\"Connected practice\")}" }
                if status.sharing_consented {
                    p { class: "record-meta", "{SHARE_WITH_MY_PHYSICIAN} ({status.consent_version.as_deref().unwrap_or(\"v1\")})" }
                    Button {
                        danger: true,
                        onclick: stop_sharing,
                        "{STOP_SHARING}"
                    }
                } else {
                    p { class: "record-meta", "Sharing stopped" }
                    Button {
                        onclick: grant_sharing,
                        "{SHARE_WITH_MY_PHYSICIAN}"
                    }
                }
            }
        }
        if !people.is_empty() {
            p { class: "status-title", "People" }
            for person in people {
                article { class: "voice-card",
                    p { class: "card-title", "{person}" }
                }
            }
        }
        match notice {
            Some(SharedNotice::NotSharing) => rsx! { p { class: "empty", "{NOT_SHARING}" } },
            Some(SharedNotice::CouldNotCheck) => rsx! {
                div { role: "alert",
                    p { class: "empty", "{SHARING_UNCHECKED}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(SharedNotice::SavedStayVisible) => rsx! {
                div { role: "alert",
                    p { class: "empty", "{SHARING_SAVED}" }
                    TryAgain { on_retry: retry }
                }
            },
            Some(SharedNotice::AwaitingAcceptance) => rsx! { p { class: "status-title", "{AWAITING}" } },
            None => rsx! {},
        }
    }
}
