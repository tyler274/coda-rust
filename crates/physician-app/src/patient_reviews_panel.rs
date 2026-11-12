//! Patient-page reviews section: saved background and requested reviews, their
//! evidence, outcomes, unfinished work, and `Stop` / `Resume` / `Try again`
//! lifecycle controls.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use physician_client::{HttpError, PhysicianClient};
use physician_wire::{
    CitedSourceWire, MessageSegment, PatientReviewOutcomeWire, PatientReviewStateWire,
    PatientReviewWire, ReviewActionWire, ReviewControlActionWire, ReviewControlRequest,
    ReviewControlResponse, ReviewEvidenceRefWire, ReviewHoldReasonWire, ReviewStepStateWire,
    ReviewStepWire, ReviewSupersededReasonWire, ReviewTriggerWire, ReviewVersion,
};
use ui::{
    ActionRow, Button, ButtonVariant, Card, IconName, NavigationRow, StatePill, StatusLine,
    StepState, Tone, WorkStep, WorkTrace, WorkingMark, WorkingPresence,
};

use crate::shell::{
    ReadView, SourceChip, TRY_AGAIN, answer_to, format_context_time, message_pieces, read_view,
};
use crate::status::{ReadNotice, ReadStage};

const REVIEWS_LOADING: &str = "Loading reviews…";
const REVIEWS_STALE: &str = "Coda couldn’t refresh reviews. Showing the last list.";
const REVIEWS_UNAVAILABLE: &str = "Coda couldn’t load reviews for this patient.";

/// Gentle-backoff poll interval while any review in the mounted workspace is
/// `Pending` or `Running`: 1.5s across the first 30s (`0..20`), 3s across the
/// next 3m (`20..80`), 5s across the rest of the 10m background run/lease window
/// (`80..200`), and 10s thereafter if multi-attempt recovery continues.
const fn working_review_poll_delay_ms(polls: u32) -> u32 {
    if polls < 20 {
        1_500
    } else if polls < 80 {
        3_000
    } else if polls < 200 {
        5_000
    } else {
        10_000
    }
}

/// Shell signals the patient reviews section reads and refreshes through.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct PatientReviewsContext {
    pub(crate) client: Signal<PhysicianClient>,
    pub(crate) session: crate::SessionState,
    pub(crate) shown: Signal<bool>,
    pub(crate) workspace_attempt: Signal<u32>,
    pub(crate) today_attempt: Signal<u32>,
}

/// Where the physician's latest lifecycle control request for one review stands.
#[derive(Clone, Debug, PartialEq, Eq)]
enum ReviewControlStatus {
    /// Request is in flight; UI waits on the server's answer without mutating the review.
    Pending {
        review_id: String,
        action: ReviewControlActionWire,
    },
    /// Server refused or transport failed the control request.
    Failed {
        review_id: String,
        action: ReviewControlActionWire,
        message: String,
        retry_request: Option<ReviewControlRequest>,
    },
    /// Server accepted the control and returned the updated review.
    Done {
        review_id: String,
        settled_version: ReviewVersion,
        action: ReviewControlActionWire,
        message: String,
    },
}

impl ReviewControlStatus {
    fn review_id(&self) -> &str {
        match self {
            Self::Pending { review_id, .. }
            | Self::Failed { review_id, .. }
            | Self::Done { review_id, .. } => review_id,
        }
    }

    fn is_pending_for(&self, target_review_id: &str) -> bool {
        matches!(
            self,
            Self::Pending { review_id, .. } if review_id == target_review_id
        )
    }

    fn applies_to(&self, target_review_id: &str, current_version: &ReviewVersion) -> bool {
        match self {
            Self::Pending { review_id, .. } | Self::Failed { review_id, .. } => {
                review_id == target_review_id
            }
            Self::Done {
                review_id,
                settled_version,
                ..
            } => review_id == target_review_id && settled_version == current_version,
        }
    }
}

/// Reads `?review=<id>` from the browser address when `?patient=` matches `expected_patient`.
#[cfg(target_arch = "wasm32")]
pub(crate) fn read_review_param(expected_patient: &str) -> Option<String> {
    let window = web_sys::window()?;
    let href = window.location().href().ok()?;
    let url = web_sys::Url::new(&href).ok()?;
    if url.search_params().get("patient").as_deref() != Some(expected_patient) {
        return None;
    }
    url.search_params()
        .get("review")
        .filter(|value| !value.trim().is_empty())
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_review_param(_expected_patient: &str) -> Option<String> {
    None
}

/// Writes or clears `?review=<id>` on the browser address when `?patient=` matches `expected_patient`.
#[cfg(target_arch = "wasm32")]
pub(crate) fn write_review_param(expected_patient: &str, review: Option<&str>) {
    if let Some(window) = web_sys::window()
        && let Ok(href) = window.location().href()
        && let Ok(url) = web_sys::Url::new(&href)
    {
        if url.search_params().get("patient").as_deref() != Some(expected_patient) {
            return;
        }
        match review.filter(|value| !value.trim().is_empty()) {
            Some(id) => url.search_params().set("review", id),
            None => url.search_params().delete("review"),
        }
        if let Ok(history) = window.history() {
            let _ =
                history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url.href()));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn write_review_param(_expected_patient: &str, _review: Option<&str>) {}

/// Resolves which review ID should be open for the loaded list: preserves `requested`
/// when present in `reviews`, otherwise selects the newest review (`reviews[0]`).
pub(crate) fn resolve_selected_review_id(
    requested: Option<&str>,
    reviews: &[PatientReviewWire],
) -> Option<String> {
    if let Some(requested_id) = requested
        && reviews
            .iter()
            .any(|review| review.review_id == requested_id)
    {
        return Some(requested_id.to_owned());
    }
    reviews.first().map(|review| review.review_id.clone())
}

/// Patient-page section rendering saved reviews and their lifecycle controls.
#[component]
pub(crate) fn PatientReviewsPanel(
    workspace_id: String,
    server_now: String,
    time_zone: String,
    context: PatientReviewsContext,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    let mut current_ws = use_signal(|| workspace_id.clone());
    if *current_ws.peek() != workspace_id {
        current_ws.set(workspace_id.clone());
    }

    let mut reviews_attempt = use_signal(|| 0u32);
    let mut last_reviews = use_signal(|| None::<(String, Vec<PatientReviewWire>)>);
    let mut rechecked_access = use_signal(|| None::<String>);
    let mut selected_review = use_signal(|| {
        let ws = workspace_id.clone();
        read_review_param(&ws).map(|review_id| (ws, review_id))
    });
    let mut control_status = use_signal(|| None::<ReviewControlStatus>);

    // Sync selection seed if the physician navigated to another patient workspace.
    use_effect(move || {
        let ws = current_ws();
        let matches_ws = selected_review
            .peek()
            .as_ref()
            .is_some_and(|(held_ws, _)| held_ws == &ws);
        if !matches_ws {
            selected_review.set(read_review_param(&ws).map(|id| (ws, id)));
            control_status.set(None);
        }
    });

    let reviews_resource = use_resource(move || {
        let ws = current_ws();
        let _ws_attempt = (context.workspace_attempt)();
        let _local_attempt = reviews_attempt();
        let client = context.client.read().clone();
        let presented = client.session().get();
        async move {
            let result = match client.patient_reviews(&ws).await {
                Ok(response) => {
                    rechecked_access.set(None);
                    let mut shown = context.shown;
                    shown.set(true);
                    Ok(response.reviews)
                }
                Err(error) => {
                    if let Some(refused) = crate::refusal(&error) {
                        last_reviews.set(None);
                        context
                            .session
                            .refused(refused, *context.shown.peek(), presented);
                    } else if matches!(
                        error,
                        HttpError::Status {
                            status: 403 | 404,
                            ..
                        }
                    ) {
                        last_reviews.set(None);
                        write_review_param(&ws, None);
                        if rechecked_access.peek().as_deref() != Some(ws.as_str()) {
                            rechecked_access.set(Some(ws.clone()));
                            let mut ws_attempt = context.workspace_attempt;
                            let mut today_attempt = context.today_attempt;
                            *ws_attempt.write() += 1;
                            *today_attempt.write() += 1;
                        }
                    }
                    Err(())
                }
            };
            (ws, result)
        }
    });

    use_effect(move || {
        if let Some((asked_ws, Ok(list))) = reviews_resource.read().as_ref() {
            last_reviews.set(Some((asked_ws.clone(), list.clone())));
        }
    });

    let ws_now = current_ws();
    let view: ReadView<Vec<PatientReviewWire>> = read_view(
        answer_to(&ws_now, last_reviews.read().as_ref()).cloned(),
        answer_to(&ws_now, reviews_resource.read().as_ref()).cloned(),
    );

    // Keep selected review valid and synced into `?review=` once the workspace's reviews arrive.
    use_effect(move || {
        let ws = current_ws();
        let Some(list) = answer_to(&ws, last_reviews.read().as_ref()).cloned() else {
            return;
        };
        let held = selected_review
            .peek()
            .as_ref()
            .filter(|(held_ws, _)| held_ws == &ws)
            .map(|(_, id)| id.clone())
            .or_else(|| read_review_param(&ws));
        let chosen = resolve_selected_review_id(held.as_deref(), &list);
        let next = chosen.clone().map(|id| (ws.clone(), id));
        if *selected_review.peek() != next {
            selected_review.set(next);
        }
        write_review_param(&ws, chosen.as_deref());
    });

    // Gentle-backoff background poll while any review on this mounted panel is working.
    let mut review_poll = use_signal(|| (0u64, 0u32));
    use_effect(move || {
        let ws = current_ws();
        let any_working = answer_to(&ws, last_reviews.read().as_ref())
            .is_some_and(|list| list.iter().any(|rev| rev.state.is_working()));
        let (generation, polls) = *review_poll.peek();
        let generation = generation + 1;
        if !any_working {
            review_poll.set((generation, 0));
            return;
        }
        let delay_ms = working_review_poll_delay_ms(polls);
        review_poll.set((generation, polls.saturating_add(1)));
        spawn(async move {
            crate::sleep_ms(delay_ms).await;
            if review_poll.peek().0 == generation {
                *reviews_attempt.write() += 1;
            }
        });
    });

    let stage = view.stage();
    let reviews = view.content().cloned().unwrap_or_default();

    // Match all other optional patient-page sections: hide cleanly when Showing an empty list,
    // while surfacing Stale and Unavailable errors so failed reads never look empty.
    if matches!(stage, ReadStage::Loading | ReadStage::Showing) && reviews.is_empty() {
        return rsx! {};
    }

    let chosen_id = resolve_selected_review_id(
        selected_review
            .read()
            .as_ref()
            .filter(|(held_ws, _)| held_ws == &workspace_id)
            .map(|(_, id)| id.as_str()),
        &reviews,
    );
    let selected = chosen_id
        .as_deref()
        .and_then(|id| reviews.iter().find(|rev| rev.review_id == id))
        .cloned();

    let send_control = move |ws: String, request: ReviewControlRequest| {
        if control_status
            .peek()
            .as_ref()
            .is_some_and(|status| status.is_pending_for(&request.review_id))
        {
            return;
        }
        let review_id = request.review_id.clone();
        let action = request.action;
        let mut status_sig = control_status;
        status_sig.set(Some(ReviewControlStatus::Pending {
            review_id: review_id.clone(),
            action,
        }));
        let client = context.client.read().clone();
        let presented = client.session().get();
        spawn(async move {
            match client.control_patient_review(&ws, &request).await {
                Ok(ReviewControlResponse::Accepted { review }) => {
                    let settled_version = review.version.clone();
                    upsert_cached_review(&mut last_reviews, &ws, *review);
                    control_status.set(Some(ReviewControlStatus::Done {
                        review_id,
                        settled_version,
                        action,
                        message: action.done_message().to_owned(),
                    }));
                    *reviews_attempt.write() += 1;
                }
                Ok(ReviewControlResponse::Refused {
                    message, review, ..
                }) => {
                    if let Some(latest) = review {
                        upsert_cached_review(&mut last_reviews, &ws, *latest);
                    }
                    control_status.set(Some(ReviewControlStatus::Failed {
                        review_id,
                        action,
                        message,
                        retry_request: None,
                    }));
                }
                Err(error) => {
                    if let Some(refused) = crate::refusal(&error) {
                        last_reviews.set(None);
                        control_status.set(None);
                        context
                            .session
                            .refused(refused, *context.shown.peek(), presented);
                    } else if matches!(
                        error,
                        HttpError::Status {
                            status: 403 | 404,
                            ..
                        }
                    ) {
                        last_reviews.set(None);
                        control_status.set(None);
                        let mut ws_attempt = context.workspace_attempt;
                        let mut today_attempt = context.today_attempt;
                        *ws_attempt.write() += 1;
                        *today_attempt.write() += 1;
                    } else {
                        control_status.set(Some(ReviewControlStatus::Failed {
                            review_id,
                            action,
                            message: control_failure_message(action, &error),
                            retry_request: Some(request),
                        }));
                    }
                }
            }
        });
    };

    rsx! {
        section {
            class: "patient-section patient-reviews-section",
            aria_label: "Patient reviews",
            "data-patient-reviews": "true",
            h2 { "Reviews" }
            ReadNotice {
                stage,
                loading: REVIEWS_LOADING,
                stale: REVIEWS_STALE,
                unavailable: REVIEWS_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *reviews_attempt.write() += 1,
            }
            if !reviews.is_empty() {
                div {
                    class: "patient-reviews-list",
                    role: "list",
                    aria_label: "Saved reviews",
                    for rev in reviews.iter() {
                        {
                            let rev_id = rev.review_id.clone();
                            let is_selected = chosen_id.as_deref() == Some(rev_id.as_str());
                            let row_state = state_slug(&rev.state);
                            let headline = row_headline(rev);
                            let subtitle = row_subtitle(rev, &time_zone, &server_now);
                            let ws_for_click = workspace_id.clone();
                            rsx! {
                                div {
                                    key: "{rev.review_id}",
                                    role: "listitem",
                                    class: "patient-review-listitem",
                                    NavigationRow {
                                        class: if is_selected {
                                            "patient-review-row is-selected"
                                        } else {
                                            "patient-review-row"
                                        },
                                        "data-review-row": "{rev.review_id}",
                                        "data-review-row-state": "{row_state}",
                                        aria_current: is_selected.then_some("true"),
                                        onclick: move |_| {
                                            if control_status
                                                .peek()
                                                .as_ref()
                                                .is_some_and(|status| status.review_id() != rev_id)
                                            {
                                                control_status.set(None);
                                            }
                                            selected_review.set(Some((ws_for_click.clone(), rev_id.clone())));
                                            write_review_param(&ws_for_click, Some(&rev_id));
                                        },
                                        span { class: "patient-review-row-head",
                                            span { class: "patient-review-row-title-group",
                                                if rev.state.is_working() {
                                                    WorkingMark {}
                                                }
                                                strong { class: "patient-review-row-title", "{headline}" }
                                            }
                                            StatePill {
                                                tone: state_tone(&rev.state),
                                                "{rev.state.badge_label()}"
                                            }
                                        }
                                        span { class: "patient-review-row-meta", "{subtitle}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(review) = selected {
                {
                    let status_for_selected = control_status
                        .read()
                        .clone()
                        .filter(|status| status.applies_to(&review.review_id, &review.version));
                    let is_busy = status_for_selected
                        .as_ref()
                        .is_some_and(|status| status.is_pending_for(&review.review_id));
                    let status_slug = state_slug(&review.state);
                    let version_str = review.version.as_str().to_owned();
                    let card_title = detail_headline(&review);
                    let card_detail = detail_timestamp(&review, &time_zone, &server_now);
                    let available_action = review.state.available_control();
                    let ws_for_action = workspace_id.clone();
                    let rev_id_for_action = review.review_id.clone();
                    let rev_version_for_action = review.version.clone();
                    let ws_for_retry = workspace_id.clone();

                    rsx! {
                        Card {
                            class: "patient-review-detail",
                            "data-selected-review-id": "{review.review_id}",
                            "data-review-version": "{version_str}",
                            "data-review-status": "{status_slug}",
                            overline: "Selected review".to_owned(),
                            mark: rsx! {
                                StatePill {
                                    tone: state_tone(&review.state),
                                    "{review.state.badge_label()}"
                                }
                            },
                            title: card_title,
                            detail: card_detail,
                            if let Some(action) = available_action {
                                ActionRow {
                                    Button {
                                        compact: true,
                                        variant: if action == ReviewControlActionWire::Resume {
                                            ButtonVariant::Primary
                                        } else {
                                            ButtonVariant::Secondary
                                        },
                                        busy: is_busy,
                                        disabled: is_busy,
                                        pending_label: action.pending_message().to_owned(),
                                        "data-review-control": "{control_slug(action)}",
                                        onclick: move |_| {
                                            let request = ReviewControlRequest {
                                                review_id: rev_id_for_action.clone(),
                                                version: rev_version_for_action.clone(),
                                                action,
                                                idempotency_key: crate::idempotency_key(),
                                            };
                                            send_control(ws_for_action.clone(), request);
                                        },
                                        "{action.button_label()}"
                                    }
                                }
                            }
                            if let Some(answer) = status_for_selected {
                                {render_control_answer(answer, ws_for_retry, send_control)}
                            }
                            div { class: "patient-review-sections",
                                {render_outcome_section(&review, on_open_source)}
                                {render_trigger_section(&review, &time_zone, &server_now)}
                                {render_evidence_section(&review, &time_zone, &server_now, on_open_source)}
                                {render_unfinished_section(&review)}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn upsert_cached_review(
    cache: &mut Signal<Option<(String, Vec<PatientReviewWire>)>>,
    workspace_id: &str,
    updated: PatientReviewWire,
) {
    let mut guard = cache.write();
    if let Some((cached_ws, list)) = guard.as_mut()
        && cached_ws == workspace_id
    {
        if let Some(slot) = list
            .iter_mut()
            .find(|existing| existing.review_id == updated.review_id)
        {
            *slot = updated;
        } else {
            list.insert(0, updated);
        }
    } else {
        *guard = Some((workspace_id.to_owned(), vec![updated]));
    }
}

fn render_control_answer(
    answer: ReviewControlStatus,
    workspace_id: String,
    send_control: impl Fn(String, ReviewControlRequest) + Copy + 'static,
) -> Element {
    match answer {
        ReviewControlStatus::Pending { action, .. } => rsx! {
            div {
                class: "patient-review-control-answer",
                "data-review-control-state": "pending",
                StatusLine {
                    message: action.pending_message().to_owned(),
                    alert: false,
                }
            }
        },
        ReviewControlStatus::Failed {
            message,
            retry_request,
            ..
        } => {
            let retry_label = if retry_request.is_some() {
                crate::CHECK_AGAIN.to_owned()
            } else {
                String::new()
            };
            let on_retry = retry_request.map(|req| {
                EventHandler::new(move |()| {
                    send_control(workspace_id.clone(), req.clone());
                })
            });
            rsx! {
                div {
                    class: "patient-review-control-answer",
                    "data-review-control-state": "failed",
                    StatusLine {
                        message,
                        alert: true,
                        retry: retry_label,
                        on_retry,
                    }
                }
            }
        }
        ReviewControlStatus::Done { message, .. } => rsx! {
            div {
                class: "patient-review-control-answer",
                "data-review-control-state": "done",
                StatusLine {
                    message,
                    alert: false,
                }
            }
        },
    }
}

fn render_outcome_section(
    review: &PatientReviewWire,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    let slug = state_slug(&review.state);
    rsx! {
        div {
            class: "patient-review-block",
            "data-review-outcome": "{slug}",
            h3 { class: "patient-review-block-heading", "Outcome" }
            match &review.state {
                PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. } => {
                    if review.steps.is_empty() {
                        rsx! {
                            WorkingPresence {
                                label: "Reviewing the chart".to_owned(),
                                activity: "Checking connected records and clinical context".to_owned(),
                            }
                        }
                    } else {
                        rsx! {
                            WorkTrace {
                                label: "Reviewing the chart".to_owned(),
                                working: true,
                                {render_work_steps(&review.steps, None)}
                            }
                        }
                    }
                }
                PatientReviewStateWire::Held {
                    reason: ReviewHoldReasonWire::Stopped,
                    partial_text,
                    ..
                } => rsx! {
                    WorkTrace {
                        label: "You stopped Coda before it finished.".to_owned(),
                        working: false,
                        detail: Some(stopped_trace_detail(&review.steps)),
                        {render_stopped_steps(&review.steps, partial_text.as_deref())}
                    }
                },
                PatientReviewStateWire::Held {
                    reason: ReviewHoldReasonWire::Limit,
                    partial_text,
                    ..
                } => rsx! {
                    p { class: "patient-review-alert-copy",
                        "Coda couldn’t finish this review before reaching its run limit."
                    }
                    if !review.steps.is_empty() || partial_text.is_some() {
                        WorkTrace {
                            label: "Steps recorded before reaching the limit".to_owned(),
                            working: false,
                            {render_stopped_steps(&review.steps, partial_text.as_deref())}
                        }
                    }
                },
                PatientReviewStateWire::Failed { reason, .. } => rsx! {
                    p { class: "patient-review-alert-copy", "Coda couldn’t finish this review." }
                    if !reason.trim().is_empty() {
                        p { class: "patient-review-muted-copy", "{reason}" }
                    }
                    if !review.steps.is_empty() {
                        WorkTrace {
                            label: "Steps recorded before failure".to_owned(),
                            working: false,
                            {render_work_steps(&review.steps, None)}
                        }
                    }
                },
                PatientReviewStateWire::Completed { outcome, .. } => {
                    render_completed_outcome(outcome, &review.actions, &review.sources, on_open_source)
                }
                PatientReviewStateWire::Superseded { reason, .. } => rsx! {
                    p { class: "patient-review-muted-copy", "{superseded_copy(reason)}" }
                },
            }
        }
    }
}

fn render_completed_outcome(
    outcome: &PatientReviewOutcomeWire,
    actions: &[ReviewActionWire],
    sources: &BTreeMap<String, CitedSourceWire>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    match outcome {
        PatientReviewOutcomeWire::Assessed {
            summary,
            segments,
            basis,
        } => {
            let body_pieces = message_pieces(segments, sources);
            let basis_pieces = message_pieces(basis, sources);
            rsx! {
                if !body_pieces.is_empty() {
                    crate::CitedMessage { pieces: body_pieces, on_open_source }
                } else {
                    crate::MessageWords { text: summary.clone() }
                }
                {render_recorded_actions(actions)}
                if !basis_pieces.is_empty() {
                    crate::AnswerBasis { pieces: basis_pieces, on_open_source }
                }
            }
        }
        PatientReviewOutcomeWire::Quiet {
            explanation,
            segments,
            basis,
        } => {
            let body_pieces = message_pieces(segments, sources);
            let basis_pieces = message_pieces(basis, sources);
            rsx! {
                p { class: "patient-review-quiet-heading", "Reviewed, nothing to do" }
                if !body_pieces.is_empty() {
                    crate::CitedMessage { pieces: body_pieces, on_open_source }
                } else {
                    crate::MessageWords { text: explanation.clone() }
                }
                {render_recorded_actions(actions)}
                if !basis_pieces.is_empty() {
                    crate::AnswerBasis { pieces: basis_pieces, on_open_source }
                }
            }
        }
    }
}

fn render_recorded_actions(actions: &[ReviewActionWire]) -> Element {
    if actions.is_empty() {
        return rsx! {};
    }
    rsx! {
        ul { class: "patient-review-actions-list",
            for (idx, action) in actions.iter().enumerate() {
                li { key: "{idx}", "{action.summary}" }
            }
        }
    }
}

fn render_work_steps(steps: &[ReviewStepWire], trailing_note: Option<&str>) -> Element {
    let partial_key = "step-partial";
    rsx! {
        for (idx, step) in steps.iter().enumerate() {
            WorkStep {
                key: "step-{idx}",
                icon: step_icon(step.state),
                label: step.label.clone(),
                state: ui_step_state(step.state),
                detail: step.detail.clone(),
            }
        }
        if let Some(note) = trailing_note.filter(|text| !text.trim().is_empty()) {
            WorkStep {
                key: "{partial_key}",
                icon: IconName::File,
                label: "Drafted notes before stopping".to_owned(),
                state: StepState::Stopped,
                detail: Some(note.trim().to_owned()),
            }
        }
    }
}

fn render_stopped_steps(steps: &[ReviewStepWire], partial_text: Option<&str>) -> Element {
    if steps.is_empty() && partial_text.is_none_or(|text| text.trim().is_empty()) {
        let early_key = "step-stopped-early";
        return rsx! {
            WorkStep {
                key: "{early_key}",
                icon: IconName::Clock,
                label: "Stopped before Coda completed its chart check".to_owned(),
                state: StepState::Stopped,
            }
        };
    }
    render_work_steps(steps, partial_text)
}

fn render_trigger_section(
    review: &PatientReviewWire,
    time_zone: &str,
    server_now: &str,
) -> Element {
    rsx! {
        div {
            class: "patient-review-block",
            "data-review-trigger": "true",
            h3 { class: "patient-review-block-heading", "Trigger" }
            ul { class: "patient-review-trigger-list",
                for (idx, trigger) in review.triggers.iter().enumerate() {
                    li { key: "trig-{idx}", "{trigger_description(trigger)}" }
                }
                for (idx, input) in review.accepted_inputs.iter().enumerate() {
                    li { key: "input-{idx}",
                        "{accepted_input_description(&input.text, &input.recorded_at, time_zone, server_now)}"
                    }
                }
            }
        }
    }
}

fn render_evidence_section(
    review: &PatientReviewWire,
    time_zone: &str,
    server_now: &str,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    rsx! {
        div {
            class: "patient-review-block",
            "data-review-evidence": "true",
            h3 { class: "patient-review-block-heading", "Evidence" }
            if review.evidence.is_empty() {
                p { class: "patient-review-muted-copy", "{empty_evidence_copy(&review.state)}" }
            } else {
                ul { class: "patient-review-evidence-list",
                    for (idx, ev) in review.evidence.iter().enumerate() {
                        {
                            let chip = evidence_source_chip(ev, &review.sources);
                            let provenance = evidence_provenance(ev, time_zone, server_now);
                            let excerpt = ev.excerpt.trim();
                            rsx! {
                                li {
                                    key: "{ev.source_key}-{idx}",
                                    class: "patient-review-evidence-item",
                                    div { class: "patient-review-evidence-head",
                                        crate::CitationChip { chip, on_open: on_open_source }
                                        span { class: "patient-review-evidence-label", "{ev.display_label}" }
                                        if !provenance.is_empty() {
                                            span { class: "patient-review-evidence-meta", "{provenance}" }
                                        }
                                    }
                                    if !excerpt.is_empty() {
                                        p { class: "patient-review-evidence-excerpt", "“{excerpt}”" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render_unfinished_section(review: &PatientReviewWire) -> Element {
    rsx! {
        div {
            class: "patient-review-block",
            "data-review-unfinished": "true",
            h3 { class: "patient-review-block-heading", "Unfinished work" }
            if review.unfinished_work.is_empty() {
                p { class: "patient-review-muted-copy", "None." }
            } else {
                ul { class: "patient-review-unfinished-list",
                    for (idx, item) in review.unfinished_work.iter().enumerate() {
                        li {
                            key: "unfinished-{idx}",
                            class: "patient-review-unfinished-item",
                            div { class: "patient-review-unfinished-head",
                                strong { "{item.topic}" }
                                if let Some(kind) = item.limit_kind {
                                    StatePill { tone: Tone::Warning, "{kind.label()}" }
                                }
                            }
                            p { class: "patient-review-unfinished-detail", "{item.detail}" }
                            if let Some(limit_detail) = item.limit_detail.as_deref().filter(|text| !text.trim().is_empty()) {
                                p { class: "patient-review-muted-copy", "{limit_detail}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn evidence_source_chip(
    evidence: &ReviewEvidenceRefWire,
    sources: &BTreeMap<String, CitedSourceWire>,
) -> SourceChip {
    let segment = [MessageSegment::Citation {
        source_key: evidence.source_key.clone(),
        number: evidence.number,
        label: evidence.display_label.clone(),
        quote: evidence.excerpt.clone(),
    }];
    for piece in message_pieces(&segment, sources) {
        if let crate::shell::SynopsisPiece::Chips(mut chips) = piece
            && let Some(chip) = chips.pop()
        {
            return chip;
        }
    }
    SourceChip {
        index: evidence.number,
        label: evidence.display_label.clone(),
        open: false,
        preview: String::new(),
        as_of: String::new(),
        open_label: None,
        open_uri: None,
    }
}

fn evidence_provenance(
    evidence: &ReviewEvidenceRefWire,
    time_zone: &str,
    server_now: &str,
) -> String {
    let mut parts = Vec::new();
    let source = evidence.source_label.trim();
    if !source.is_empty() {
        parts.push(source.to_owned());
    }
    if let Some(as_of) = evidence.as_of.as_deref() {
        if let Some(formatted) = format_context_time(as_of, time_zone, server_now) {
            parts.push(format!("As of {formatted}"));
        } else if !as_of.trim().is_empty() {
            parts.push(format!("As of {}", as_of.trim()));
        }
    }
    parts.join(" · ")
}

fn empty_evidence_copy(state: &PatientReviewStateWire) -> &'static str {
    match state {
        PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. } => {
            "Checking connected chart records…"
        }
        PatientReviewStateWire::Held { .. } | PatientReviewStateWire::Failed { .. } => {
            "No chart records were cited before the review stopped."
        }
        PatientReviewStateWire::Completed { .. } | PatientReviewStateWire::Superseded { .. } => {
            "No chart records cited."
        }
    }
}

fn stopped_trace_detail(steps: &[ReviewStepWire]) -> String {
    match steps.len() {
        0 => "View details".to_owned(),
        1 => "1 step".to_owned(),
        count => format!("{count} steps"),
    }
}

fn step_icon(state: ReviewStepStateWire) -> IconName {
    match state {
        ReviewStepStateWire::Running => IconName::Search,
        ReviewStepStateWire::Done => IconName::Check,
        ReviewStepStateWire::Failed => IconName::Alert,
        ReviewStepStateWire::Stopped => IconName::Clock,
    }
}

fn ui_step_state(state: ReviewStepStateWire) -> StepState {
    match state {
        ReviewStepStateWire::Running => StepState::Running,
        ReviewStepStateWire::Done => StepState::Done,
        ReviewStepStateWire::Failed => StepState::Failed,
        ReviewStepStateWire::Stopped => StepState::Stopped,
    }
}

fn state_tone(state: &PatientReviewStateWire) -> Tone {
    match state {
        PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. } => Tone::Primary,
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Stopped,
            ..
        } => Tone::Neutral,
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Limit,
            ..
        }
        | PatientReviewStateWire::Failed { .. } => Tone::Error,
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Assessed { .. },
            ..
        } => Tone::Primary,
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Quiet { .. },
            ..
        }
        | PatientReviewStateWire::Superseded { .. } => Tone::Neutral,
    }
}

fn state_slug(state: &PatientReviewStateWire) -> &'static str {
    match state {
        PatientReviewStateWire::Pending => "pending",
        PatientReviewStateWire::Running { .. } => "running",
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Stopped,
            ..
        } => "held_stopped",
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Limit,
            ..
        } => "held_limit",
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Assessed { .. },
            ..
        } => "completed_assessed",
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Quiet { .. },
            ..
        } => "completed_quiet",
        PatientReviewStateWire::Failed { .. } => "failed",
        PatientReviewStateWire::Superseded { .. } => "superseded",
    }
}

fn control_slug(action: ReviewControlActionWire) -> &'static str {
    match action {
        ReviewControlActionWire::Stop => "stop",
        ReviewControlActionWire::Resume => "resume",
        ReviewControlActionWire::TryAgain => "try_again",
    }
}

fn control_failure_message(action: ReviewControlActionWire, error: &HttpError) -> String {
    let verb = match action {
        ReviewControlActionWire::Stop => "stop",
        ReviewControlActionWire::Resume => "resume",
        ReviewControlActionWire::TryAgain => "retry",
    };
    match error {
        HttpError::Status { body, .. } => serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|val| {
                val.get("message")
                    .or_else(|| val.get("detail"))
                    .and_then(serde_json::Value::as_str)
                    .filter(|msg| !msg.trim().is_empty())
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| format!("Coda couldn’t {verb} this review.")),
        HttpError::Transport(_) => {
            format!("Coda couldn’t confirm the request to {verb} this review.")
        }
    }
}

pub(crate) fn row_headline(review: &PatientReviewWire) -> String {
    if review.state.is_working() {
        return format!("Reviewing · {}", primary_trigger_headline(&review.triggers));
    }
    primary_trigger_headline(&review.triggers)
}

fn detail_headline(review: &PatientReviewWire) -> String {
    primary_trigger_headline(&review.triggers)
}

fn detail_timestamp(
    review: &PatientReviewWire,
    time_zone: &str,
    server_now: &str,
) -> Option<String> {
    let label = if review.state.is_working() {
        "Started"
    } else {
        "Updated"
    };
    format_context_time(&review.updated_at, time_zone, server_now)
        .map(|when| format!("{label} {when}"))
}

fn row_subtitle(review: &PatientReviewWire, time_zone: &str, server_now: &str) -> String {
    let mut parts = Vec::new();
    match &review.state {
        PatientReviewStateWire::Pending | PatientReviewStateWire::Running { .. } => {
            parts.push("Reviewing the chart".to_owned());
        }
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Stopped,
            ..
        } => {
            parts.push("You stopped Coda before it finished.".to_owned());
        }
        PatientReviewStateWire::Held {
            reason: ReviewHoldReasonWire::Limit,
            ..
        }
        | PatientReviewStateWire::Failed { .. } => {
            parts.push("Coda couldn’t finish this review.".to_owned());
        }
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Assessed { summary, .. },
            ..
        } => {
            let line = first_sentence(summary);
            if !line.is_empty() {
                parts.push(line);
            }
        }
        PatientReviewStateWire::Completed {
            outcome: PatientReviewOutcomeWire::Quiet { .. },
            ..
        } => {
            parts.push("No concerns or changes needed.".to_owned());
        }
        PatientReviewStateWire::Superseded { reason, .. } => {
            parts.push(superseded_copy(reason));
        }
    }
    if let Some(when) = format_context_time(&review.updated_at, time_zone, server_now) {
        parts.push(when);
    }
    parts.join(" · ")
}

fn first_sentence(text: &str) -> String {
    let trimmed = text.trim();
    let line = trimmed.lines().next().unwrap_or_default().trim();
    if line.chars().count() <= 96 {
        line.to_owned()
    } else {
        let shortened: String = line.chars().take(95).collect();
        format!("{shortened}…")
    }
}

fn primary_trigger_headline(triggers: &[ReviewTriggerWire]) -> String {
    let Some(first) = triggers.first() else {
        return "Chart review".to_owned();
    };
    match first {
        ReviewTriggerWire::ChangedRecords {
            changed_records, ..
        } => {
            if let Some(title) = changed_records
                .iter()
                .map(|t| t.trim())
                .find(|t| !t.is_empty())
            {
                if changed_records.len() > 1 {
                    format!("New records: {title} (+{})", changed_records.len() - 1)
                } else {
                    format!("New records: {title}")
                }
            } else {
                "New records".to_owned()
            }
        }
        ReviewTriggerWire::Periodic { .. } => "Periodic chart review".to_owned(),
        ReviewTriggerWire::PhysicianRequest { request_text, .. } => {
            let trimmed = request_text.trim();
            if trimmed.is_empty() {
                "Requested review".to_owned()
            } else {
                first_sentence(trimmed)
            }
        }
    }
}

fn trigger_description(trigger: &ReviewTriggerWire) -> String {
    match trigger {
        ReviewTriggerWire::ChangedRecords {
            changed_records,
            workspace_version,
        } => {
            let named: Vec<&str> = changed_records
                .iter()
                .map(String::as_str)
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .collect();
            if !named.is_empty() {
                format!("New records: {}", named.join(", "))
            } else {
                format!("New records (chart revision {workspace_version})")
            }
        }
        ReviewTriggerWire::Periodic {
            last_reviewed_at, ..
        } => match last_reviewed_at.as_deref().filter(|t| !t.trim().is_empty()) {
            Some(prior) => format!("Scheduled periodic chart review (prior review {prior})"),
            None => "Scheduled periodic chart review".to_owned(),
        },
        ReviewTriggerWire::PhysicianRequest { request_text, .. } => {
            format!("Physician request: {}", request_text.trim())
        }
    }
}

fn accepted_input_description(
    text: &str,
    recorded_at: &str,
    time_zone: &str,
    server_now: &str,
) -> String {
    match format_context_time(recorded_at, time_zone, server_now) {
        Some(when) => format!("Physician note ({when}): {}", text.trim()),
        None => format!("Physician note: {}", text.trim()),
    }
}

fn superseded_copy(reason: &ReviewSupersededReasonWire) -> String {
    match reason {
        ReviewSupersededReasonWire::AccessLost { detail } => {
            let trimmed = detail.trim();
            if trimmed.is_empty() {
                "Practice access changed before this review finished.".to_owned()
            } else {
                format!("Superseded: {trimmed}")
            }
        }
        ReviewSupersededReasonWire::NewInput {
            replacement_review_id,
        } => match replacement_review_id
            .as_deref()
            .filter(|id| !id.trim().is_empty())
        {
            Some(id) => format!("A newer review replaced this run ({id})."),
            None => "Newer chart input superseded this review.".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use physician_wire::{
        PatientReviewOutcomeWire, PatientReviewStateWire, PatientReviewWire,
        ReviewControlActionWire, ReviewHoldReasonWire, ReviewTriggerWire, ReviewVersion,
    };

    use super::{
        ReviewControlStatus, resolve_selected_review_id, row_headline, working_review_poll_delay_ms,
    };

    fn sample_review(id: &str, state: PatientReviewStateWire) -> PatientReviewWire {
        PatientReviewWire {
            review_id: id.to_owned(),
            version: ReviewVersion(format!("rv-{id}")),
            work_id: format!("work-{id}"),
            triggers: vec![ReviewTriggerWire::ChangedRecords {
                workspace_version: 3,
                changed_records: vec!["CMP — Potassium 5.6".to_owned()],
            }],
            accepted_inputs: Vec::new(),
            state,
            steps: Vec::new(),
            evidence: Vec::new(),
            sources: Default::default(),
            actions: Vec::new(),
            unfinished_work: Vec::new(),
            queued_at: "2026-08-27T14:00:00Z".to_owned(),
            updated_at: "2026-08-27T14:05:00Z".to_owned(),
        }
    }

    #[test]
    fn selection_keeps_requested_review_across_reload_or_falls_back_to_newest() {
        let reviews = vec![
            sample_review(
                "rev-newest",
                PatientReviewStateWire::Running {
                    attempt_id: "att-1".to_owned(),
                    started_at: "2026-08-27T14:01:00Z".to_owned(),
                },
            ),
            sample_review(
                "rev-stopped",
                PatientReviewStateWire::Held {
                    reason: ReviewHoldReasonWire::Stopped,
                    held_at: "2026-08-27T14:03:00Z".to_owned(),
                    partial_text: None,
                },
            ),
        ];

        assert_eq!(
            resolve_selected_review_id(Some("rev-stopped"), &reviews).as_deref(),
            Some("rev-stopped")
        );
        assert_eq!(
            resolve_selected_review_id(Some("rev-missing"), &reviews).as_deref(),
            Some("rev-newest")
        );
        assert_eq!(
            resolve_selected_review_id(None, &reviews).as_deref(),
            Some("rev-newest")
        );
    }

    #[test]
    fn each_review_state_maps_to_its_expected_control_and_working_row_style() {
        let running = sample_review(
            "rev-run",
            PatientReviewStateWire::Running {
                attempt_id: "att-1".to_owned(),
                started_at: "2026-08-27T14:01:00Z".to_owned(),
            },
        );
        assert_eq!(
            running.state.available_control(),
            Some(ReviewControlActionWire::Stop)
        );
        assert!(row_headline(&running).starts_with("Reviewing · "));

        let stopped = sample_review(
            "rev-stop",
            PatientReviewStateWire::Held {
                reason: ReviewHoldReasonWire::Stopped,
                held_at: "2026-08-27T14:03:00Z".to_owned(),
                partial_text: None,
            },
        );
        assert_eq!(
            stopped.state.available_control(),
            Some(ReviewControlActionWire::Resume)
        );

        let failed = sample_review(
            "rev-fail",
            PatientReviewStateWire::Failed {
                reason: "Provider timeout".to_owned(),
                failed_at: "2026-08-27T14:04:00Z".to_owned(),
            },
        );
        assert_eq!(
            failed.state.available_control(),
            Some(ReviewControlActionWire::TryAgain)
        );

        let completed = sample_review(
            "rev-done",
            PatientReviewStateWire::Completed {
                completed_at: "2026-08-27T14:05:00Z".to_owned(),
                outcome: PatientReviewOutcomeWire::Quiet {
                    explanation: "Stable".to_owned(),
                    segments: Vec::new(),
                    basis: Vec::new(),
                },
            },
        );
        assert_eq!(completed.state.available_control(), None);
    }

    #[test]
    fn done_control_confirmation_retires_once_review_version_advances() {
        let settled = ReviewVersion("rv-resumed-1".to_owned());
        let later = ReviewVersion("rv-held-limit-2".to_owned());
        let done = ReviewControlStatus::Done {
            review_id: "rev-1".to_owned(),
            settled_version: settled.clone(),
            action: ReviewControlActionWire::Resume,
            message: ReviewControlActionWire::Resume.done_message().to_owned(),
        };

        assert!(done.applies_to("rev-1", &settled));
        assert!(!done.applies_to("rev-1", &later));
        assert!(!done.applies_to("rev-other", &settled));
    }

    #[test]
    fn working_review_poll_backoff_stays_active_across_ten_minute_run_window() {
        assert_eq!(working_review_poll_delay_ms(0), 1_500);
        assert_eq!(working_review_poll_delay_ms(19), 1_500);
        assert_eq!(working_review_poll_delay_ms(20), 3_000);
        assert_eq!(working_review_poll_delay_ms(79), 3_000);
        assert_eq!(working_review_poll_delay_ms(80), 5_000);
        assert_eq!(working_review_poll_delay_ms(199), 5_000);
        assert_eq!(working_review_poll_delay_ms(200), 10_000);
        let first_two_hundred_ms: u64 = (0..200)
            .map(|poll| u64::from(working_review_poll_delay_ms(poll)))
            .sum();
        assert!(
            first_two_hundred_ms >= 600_000,
            "first 200 polls cover >10m background run window ({first_two_hundred_ms}ms)"
        );
    }
}
