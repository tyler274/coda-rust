//! Today and Updates UI state, banners, work-and-evidence inspection card, and
//! grouped Updates surface (`What Coda did`, `Reviewed, nothing for you`, and
//! `Earlier days`).

use dioxus::prelude::*;
use physician_wire::today_updates::{
    FailedProducerWire, ItemCitationWire, ItemWorkEvidenceWire, PracticeOutageWire,
    SeenStandingWire, UpdateItemKindWire, UpdateItemWire,
};
use physician_wire::{TodayListResponse, UpdatesResponse};
use ui::{
    ActionRow, Button, ButtonVariant, Card, CitationChip, Count, IconName, NavigationRow, Notice,
    StatePill, Tone, WorkingMark,
};

use crate::shell::{ReadView, SourceChip, TRY_AGAIN, TodayLine};
use crate::status::ReadNotice;

const UPDATES_LOADING: &str = "Loading updates…";
const UPDATES_EMPTY: &str = "Nothing from Coda yet today.";
const UPDATES_EMPTY_DETAIL: &str = "What Coda does for your patients today will appear here.";
const UPDATES_UNAVAILABLE: &str = "Coda couldn’t load Updates.";
const UPDATES_STALE: &str = "Coda couldn’t refresh Updates. Showing the last list.";

/// Completeness, outage, and quiet-review metadata accompanying a Today read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct TodayMeta {
    /// Whether one or more producers failed to answer on this read.
    pub incomplete: bool,
    /// Producer parts that failed while healthy parts returned rows.
    pub failed_parts: Vec<FailedProducerWire>,
    /// Practice-wide connected source outages collapsed to one line per source.
    pub practice_outages: Vec<PracticeOutageWire>,
    /// Distinct authorized patients Coda quietly reviewed today with nothing needing the physician.
    pub also_reviewed: usize,
    /// Count of patient messages currently waiting in `Messages to match`.
    pub messages_to_match_count: usize,
    /// Whether any unchecked patient was skipped on this list (`A1.3`).
    pub may_be_missing_someone: bool,
}

impl TodayMeta {
    /// Projects metadata from a `GET /today` wire payload.
    pub fn from_response(list: &TodayListResponse) -> Self {
        Self {
            incomplete: list.incomplete == Some(true) || !list.failed_parts.is_empty(),
            failed_parts: list.failed_parts.clone(),
            practice_outages: list.practice_outages.clone(),
            also_reviewed: list.also_reviewed,
            messages_to_match_count: list.messages_to_match_count,
            may_be_missing_someone: list.may_be_missing_someone,
        }
    }
}

/// Single-owner UI predicate: `"Nothing needs you right now."` appears strictly
/// when every producer answered cleanly and Today carries no rows, connections,
/// or practice-wide source outages.
pub(crate) fn today_shows_nothing_needs_you(
    rows: &[TodayLine],
    connections: &[String],
    source_connections: &[physician_wire::TodayConnection],
    meta: &TodayMeta,
) -> bool {
    rows.is_empty()
        && connections.is_empty()
        && source_connections.is_empty()
        && !meta.incomplete
        && meta.failed_parts.is_empty()
        && meta.practice_outages.is_empty()
        && !meta.may_be_missing_someone
}

/// Formats one concise sentence naming which parts of Today or Updates could not load.
pub(crate) fn partial_failure_sentence(surface: &str, parts: &[FailedProducerWire]) -> String {
    let names: Vec<&str> = parts
        .iter()
        .map(|part| part.label.trim())
        .filter(|label| !label.is_empty())
        .collect();
    if names.is_empty() {
        format!("Part of {surface} couldn’t load. Showing available items.")
    } else {
        format!(
            "Part of {surface} couldn’t load ({}). Showing available items.",
            names.join(", ")
        )
    }
}

/// Formats one practice-wide source outage line.
pub(crate) fn practice_outage_sentence(outage: &PracticeOutageWire) -> String {
    let count = outage.affected_workspace_count;
    let noun = if count == 1 { "patient" } else { "patients" };
    if outage.failure_detail.trim().is_empty() {
        format!(
            "{} is unavailable across the practice ({count} {noun}).",
            outage.source_label
        )
    } else {
        format!(
            "{} is unavailable across the practice ({count} {noun}): {}",
            outage.source_label, outage.failure_detail
        )
    }
}

/// Target item requested when opening a Today row or an Update entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct OpenItemTarget {
    /// Patient workspace identifier owning the item.
    pub workspace_id: String,
    /// Durable seen-state item key.
    pub item_id: String,
    /// Human-readable item title for failure/retry messaging.
    pub label: String,
    /// Once-only command key reused across retries of the same open attempt.
    pub idempotency_key: String,
}

/// Classification of an item-open failure determining copy and retry eligibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ItemOpenFailureKind {
    /// Item was deleted or removed from the physician's panel (`403` / `404`).
    UnavailableOnPanel,
    /// Transient load or server failure (`Retry` stays available while older content remains).
    Retryable,
}

impl ItemOpenFailureKind {
    /// Maps an HTTP client error when opening a Today/Updates item into its failure kind.
    pub(crate) const fn from_http_error(error: &physician_client::HttpError) -> Self {
        if matches!(
            error,
            physician_client::HttpError::Status {
                status: 403 | 404,
                ..
            }
        ) {
            Self::UnavailableOnPanel
        } else {
            Self::Retryable
        }
    }
}

/// Failure state preserved when opening an item fails, keeping existing content on screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FailedItemOpen {
    /// Target that failed to open so `Retry` re-attempts the exact item.
    pub target: OpenItemTarget,
    /// Physician-facing explanation of the failure.
    pub message: String,
    /// Whether `Retry` should be offered (`true` for transient/server errors).
    pub retryable: bool,
}

/// Constructs [`FailedItemOpen`] for the classified failure kind (`P2.Part of Today can't load`).
pub(crate) fn build_item_open_failure(
    target: OpenItemTarget,
    kind: ItemOpenFailureKind,
) -> FailedItemOpen {
    match kind {
        ItemOpenFailureKind::UnavailableOnPanel => FailedItemOpen {
            target,
            message: "This item is no longer available on your panel.".to_owned(),
            retryable: false,
        },
        ItemOpenFailureKind::Retryable => FailedItemOpen {
            target,
            message: "Couldn’t load this".to_owned(),
            retryable: true,
        },
    }
}

/// State of an in-flight or failed `Done` action on Today or a patient page.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum DoneActionState {
    /// No `Done` mutation is currently running or failed.
    #[default]
    Idle,
    /// Waiting for server confirmation of `Done` on `item_id`.
    Clearing {
        /// Item currently being marked done.
        item_id: String,
    },
    /// Server rejected or failed `Done`; row remains open on screen.
    Failed {
        /// Item whose `Done` request failed.
        item_id: String,
        /// Physician-facing error message.
        message: String,
    },
}

impl DoneActionState {
    /// Whether `item_id` currently has an in-flight `Done` request.
    pub fn is_clearing(&self, item_id: &str) -> bool {
        matches!(self, Self::Clearing { item_id: active } if active == item_id)
    }
}

/// Converts a verified [`ItemCitationWire`] into a [`SourceChip`] for `ReaderDialog` inspection.
pub(crate) fn item_citation_to_chip(index: usize, citation: &ItemCitationWire) -> SourceChip {
    let open_label = citation.native_uri.as_ref().map(|_| {
        if citation.source_label.trim().is_empty() {
            "Open original".to_owned()
        } else {
            format!("Open in {}", citation.source_label)
        }
    });
    let full_record = if citation.body.trim().is_empty() {
        citation.excerpt.clone()
    } else {
        citation.body.clone()
    };
    SourceChip {
        index,
        source_key: None,
        open: true,
        label: citation.label.clone(),
        target: if citation.excerpt.trim().is_empty() {
            physician_wire::CitationTargetKind::WholeRecord
        } else {
            physician_wire::CitationTargetKind::Passage
        },
        quote: citation.excerpt.clone(),
        as_of: citation.as_of.clone().unwrap_or_default(),
        preview: full_record,
        record_kind: None,
        record_status: None,
        readable_parts: Vec::new(),
        fields: Vec::new(),
        raw_content: None,
        open_label,
        open_uri: citation.native_uri.clone(),
        origin: physician_wire::CitationOrigin::SourceRecord,
    }
}

/// One partial-load warning banner with a single `Retry` control when a subset of producers failed.
#[component]
pub(crate) fn TodayPartialFailureBanner(
    surface: &'static str,
    failed_parts: Vec<FailedProducerWire>,
    incomplete: bool,
    on_retry: EventHandler,
) -> Element {
    if !incomplete && failed_parts.is_empty() {
        return rsx! {};
    }
    let title = partial_failure_sentence(surface, &failed_parts);
    rsx! {
        div {
            class: "tu-partial-banner",
            "data-today-partial-failure": "true",
            Notice {
                tone: Tone::Warning,
                icon: Some(IconName::Alert),
                title,
                ActionRow {
                    Button {
                        compact: true,
                        "data-today-partial-retry": "true",
                        onclick: move |_| on_retry.call(()),
                        "Retry"
                    }
                }
            }
        }
    }
}

/// One collapsed outage line per practice-wide down source.
#[component]
pub(crate) fn TodayPracticeOutages(outages: Vec<PracticeOutageWire>) -> Element {
    if outages.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "tu-outage-list",
            for outage in outages {
                p {
                    class: "tu-outage-line",
                    role: "status",
                    "data-practice-outage": "{outage.source_id}",
                    "{practice_outage_sentence(&outage)}"
                }
            }
        }
    }
}

/// Inline error notice shown when opening a Today row or Update item fails while preserving older content.
#[component]
pub(crate) fn ItemOpenFailureBanner(
    failure: Option<FailedItemOpen>,
    opening_busy: bool,
    on_retry: EventHandler<OpenItemTarget>,
    on_dismiss: EventHandler,
) -> Element {
    let Some(failure) = failure else {
        return rsx! {};
    };
    let retry_target = failure.target.clone();
    rsx! {
        div {
            class: "tu-open-failure",
            "data-item-open-failure": "true",
            Notice {
                tone: Tone::Error,
                icon: Some(IconName::Alert),
                title: failure.message,
                detail: Some("Existing content is still shown.".to_owned()),
                ActionRow {
                    if failure.retryable {
                        Button {
                            compact: true,
                            disabled: opening_busy,
                            "data-item-open-retry": "true",
                            onclick: move |_| on_retry.call(retry_target.clone()),
                            "Retry"
                        }
                    }
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| on_dismiss.call(()),
                        "Dismiss"
                    }
                }
            }
        }
    }
}

/// Inline error notice when clearing an item as `Done` fails.
#[component]
pub(crate) fn DoneFailureBanner(state: DoneActionState, on_dismiss: EventHandler) -> Element {
    let DoneActionState::Failed { message, .. } = state else {
        return rsx! {};
    };
    rsx! {
        div {
            class: "tu-done-failure",
            "data-done-failure": "true",
            Notice {
                tone: Tone::Error,
                icon: Some(IconName::Alert),
                title: message,
                ActionRow {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| on_dismiss.call(()),
                        "Dismiss"
                    }
                }
            }
        }
    }
}

/// Quiet summary bar below Today rows linking to quiet reviews in Updates.
#[component]
pub(crate) fn TodayAlsoReviewedBar(also_reviewed: usize, on_open_updates: EventHandler) -> Element {
    if also_reviewed == 0 {
        return rsx! {};
    }
    let noun = if also_reviewed == 1 {
        "patient"
    } else {
        "patients"
    };
    rsx! {
        NavigationRow {
            class: "tu-also-reviewed",
            "data-also-reviewed": "{also_reviewed}",
            onclick: move |_| on_open_updates.call(()),
            span { class: "tu-also-reviewed-copy",
                "Coda also reviewed {also_reviewed} {noun} today — nothing needed you"
            }
            StatePill { tone: Tone::Neutral, "View in Updates" }
        }
    }
}

/// Inline card on the patient page presenting the work and verified chart citations of an opened Today/Updates item.
#[component]
pub(crate) fn OpenedWorkEvidencePanel(
    evidence: ItemWorkEvidenceWire,
    can_mark_done: bool,
    done_busy: bool,
    on_mark_done: EventHandler,
    on_open_source: EventHandler<SourceChip>,
    on_close: EventHandler,
) -> Element {
    let item_id = evidence.item_id.clone();
    rsx! {
        section {
            class: "patient-section tu-evidence-panel",
            aria_label: "Opened work and evidence",
            "data-work-evidence": "{item_id}",
            Card {
                mark: rsx! { StatePill { tone: Tone::Primary, "{evidence.kind_label}" } },
                title: evidence.headline.clone(),
                if !evidence.narrative.trim().is_empty()
                    && evidence.narrative.trim() != evidence.headline.trim()
                {
                    p { class: "tu-evidence-narrative", "{evidence.narrative}" }
                }
                if let Some(basis) = evidence
                    .basis_and_limitations
                    .as_ref()
                    .filter(|text| !text.trim().is_empty())
                {
                    div { class: "tu-evidence-basis",
                        strong { class: "tu-evidence-subheading", "Basis and limitations" }
                        p { "{basis}" }
                    }
                }
                if !evidence.citations.is_empty() {
                    div { class: "tu-evidence-citations",
                        strong { class: "tu-evidence-subheading", "Chart evidence" }
                        for (index, citation) in evidence.citations.iter().enumerate() {
                            div {
                                class: "tu-evidence-citation",
                                "data-citation-candidate": "{citation.candidate_id}",
                                div { class: "tu-evidence-citation-head",
                                    {
                                        let chip = item_citation_to_chip(index + 1, citation);
                                        let chip_label = chip.label.clone();
                                        rsx! {
                                            CitationChip {
                                                index: index + 1,
                                                label: chip_label,
                                                onclick: move |_| on_open_source.call(chip.clone()),
                                            }
                                        }
                                    }
                                    span { class: "tu-evidence-citation-meta",
                                        if let Some(as_of) = citation.as_of.as_deref().filter(|d| !d.is_empty()) {
                                            "{citation.source_label} · {as_of}"
                                        } else {
                                            "{citation.source_label}"
                                        }
                                    }
                                }
                                if !citation.excerpt.trim().is_empty() {
                                    blockquote { class: "tu-evidence-excerpt", "{citation.excerpt}" }
                                }
                            }
                        }
                    }
                }
                ActionRow {
                    if can_mark_done {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            disabled: done_busy,
                            "data-evidence-done": "{evidence.item_id}",
                            onclick: move |_| on_mark_done.call(()),
                            if done_busy { "Saving…" } else { "Done" }
                        }
                    }
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}

fn update_kind_label(kind: UpdateItemKindWire) -> &'static str {
    match kind {
        UpdateItemKindWire::ConcernRaised => "Concern raised",
        UpdateItemKindWire::ConcernRetired => "Concern retired",
        UpdateItemKindWire::AssessedReview => "Chart review",
        UpdateItemKindWire::QuietReview => "Reviewed, nothing for you",
        UpdateItemKindWire::VisitBriefing => "Visit briefing",
        UpdateItemKindWire::PatientSummary => "Patient summary",
        UpdateItemKindWire::EndedNothingToDo => "Skipped",
        UpdateItemKindWire::FollowUpChanged => "Follow-up",
    }
}

fn update_kind_tone(kind: UpdateItemKindWire) -> Tone {
    match kind {
        UpdateItemKindWire::ConcernRaised => Tone::Warning,
        UpdateItemKindWire::AssessedReview
        | UpdateItemKindWire::VisitBriefing
        | UpdateItemKindWire::PatientSummary
        | UpdateItemKindWire::FollowUpChanged => Tone::Primary,
        UpdateItemKindWire::ConcernRetired
        | UpdateItemKindWire::QuietReview
        | UpdateItemKindWire::EndedNothingToDo => Tone::Neutral,
    }
}

#[component]
fn UpdateItemRow(item: UpdateItemWire, on_open: EventHandler<OpenItemTarget>) -> Element {
    let changed = match &item.standing {
        SeenStandingWire::ChangedSinceSeen { what_changed, .. } => Some(what_changed.clone()),
        SeenStandingWire::Unseen | SeenStandingWire::Seen { .. } => None,
    };
    let target = OpenItemTarget {
        workspace_id: item.patient_workspace_id.clone(),
        item_id: item.item_id.clone(),
        label: item.headline.clone(),
        idempotency_key: crate::idempotency_key(),
    };
    let detail_trimmed = item.detail.trim();
    let headline_trimmed = item.headline.trim();
    let show_detail = !detail_trimmed.is_empty()
        && detail_trimmed != headline_trimmed
        && !headline_trimmed.ends_with(detail_trimmed);
    rsx! {
        article {
            class: "today-row",
            "data-update-item": "{item.item_id}",
            NavigationRow {
                class: "today-row-open",
                onclick: move |_| on_open.call(target.clone()),
                span { class: "today-row-line",
                    span { class: "today-row-heading",
                        strong { class: "today-row-who", "{item.patient_display_name}" }
                        StatePill {
                            tone: update_kind_tone(item.kind),
                            "{update_kind_label(item.kind)}"
                        }
                        if let Some(what_changed) = changed {
                            span {
                                class: "tu-changed-badge",
                                "data-changed-since-seen": "true",
                                "Updated · {what_changed}"
                            }
                        }
                    }
                }
                span { class: "today-row-reason", "{item.headline}" }
                if show_detail {
                    span { class: "tu-update-detail", "{item.detail}" }
                }
            }
        }
    }
}

/// Full Updates surface showing active runs, what Coda did today, the
/// `"Reviewed, nothing for you"` group, and earlier days.
#[component]
pub(crate) fn UpdatesMainSurface(
    state: ReadView<UpdatesResponse>,
    open_failure: Option<FailedItemOpen>,
    opening_busy: bool,
    on_retry: EventHandler,
    on_retry_open: EventHandler<OpenItemTarget>,
    on_dismiss_open_failure: EventHandler,
    on_open_patient: EventHandler<String>,
    on_open_item: EventHandler<OpenItemTarget>,
) -> Element {
    let mut quiet_expanded = use_signal(|| true);
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading", h1 { "Updates" } }
            ReadNotice {
                stage: state.stage(),
                loading: UPDATES_LOADING,
                stale: UPDATES_STALE,
                unavailable: UPDATES_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            ItemOpenFailureBanner {
                failure: open_failure,
                opening_busy,
                on_retry: on_retry_open,
                on_dismiss: on_dismiss_open_failure,
            }
            match state.content().cloned() {
                None => rsx! {},
                Some(page)
                    if page.active_runs.is_empty()
                        && page.today_items.is_empty()
                        && page.rows.is_empty()
                        && page.reviewed_nothing_for_you.is_empty()
                        && page.earlier_days.is_empty()
                        && page.failed_parts.is_empty()
                        && !page.may_be_missing_someone =>
                {
                    rsx! {
                        div { class: "updates-empty",
                            h2 { "{UPDATES_EMPTY}" }
                            p { "{UPDATES_EMPTY_DETAIL}" }
                        }
                    }
                }
                Some(page) => rsx! {
                    div { class: "today-main",
                        crate::access_notice::MayBeMissingSomeoneNotice {
                            visible: page.may_be_missing_someone,
                        }
                        TodayPartialFailureBanner {
                            surface: "Updates",
                            failed_parts: page.failed_parts.clone(),
                            incomplete: !page.failed_parts.is_empty(),
                            on_retry,
                        }
                        if !page.active_runs.is_empty() {
                            section {
                                class: "tu-updates-group",
                                aria_label: "Running now",
                                "data-updates-group": "active-runs",
                                h2 { class: "tu-updates-heading", "Running now" }
                                div { class: "today-list",
                                    for run in page.active_runs {
                                        article { class: "today-row",
                                            NavigationRow {
                                                class: "today-row-open",
                                                onclick: {
                                                    let id = run.patient_workspace_id.clone();
                                                    move |_| on_open_patient.call(id.clone())
                                                },
                                                span { class: "today-row-heading",
                                                    WorkingMark {}
                                                    strong { class: "today-row-who", "{run.patient_display_name}" }
                                                    StatePill { tone: Tone::Primary, "{run.headline}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !page.today_items.is_empty() {
                            section {
                                class: "tu-updates-group",
                                aria_label: "Today",
                                "data-updates-group": "today-items",
                                h2 { class: "tu-updates-heading", "What Coda did today" }
                                div { class: "today-list",
                                    for item in page.today_items {
                                        UpdateItemRow { item, on_open: on_open_item }
                                    }
                                }
                            }
                        } else if !page.rows.is_empty() {
                            div { class: "today-list",
                                for row in page.rows {
                                    article { class: "today-row",
                                        if let Some(workspace_id) = row
                                            .get("patient_workspace_id")
                                            .and_then(|value| value.as_str())
                                            .map(str::to_owned)
                                        {
                                            NavigationRow {
                                                class: "today-row-open",
                                                onclick: {
                                                    let item_id = row
                                                        .get("item_id")
                                                        .and_then(|value| value.as_str())
                                                        .map(str::to_owned);
                                                    let headline = row
                                                        .get("headline")
                                                        .or_else(|| row.get("summary"))
                                                        .and_then(|value| value.as_str())
                                                        .unwrap_or("Update")
                                                        .to_owned();
                                                    move |_| {
                                                        if let Some(item_id) = item_id.clone() {
                                                            on_open_item.call(OpenItemTarget {
                                                                workspace_id: workspace_id.clone(),
                                                                item_id,
                                                                label: headline.clone(),
                                                                idempotency_key: crate::idempotency_key(),
                                                            });
                                                        } else {
                                                            on_open_patient.call(workspace_id.clone());
                                                        }
                                                    }
                                                },
                                                strong { class: "today-row-who",
                                                    "{row.get(\"patient_display_name\").and_then(|value| value.as_str()).unwrap_or(\"Update\")}"
                                                }
                                                if let Some(detail) = row
                                                    .get("headline")
                                                    .or_else(|| row.get("summary"))
                                                    .or_else(|| row.get("reason"))
                                                    .and_then(|value| value.as_str())
                                                    .filter(|text| !text.is_empty())
                                                {
                                                    span { class: "today-row-reason", "{detail}" }
                                                }
                                            }
                                        } else {
                                            strong { class: "today-row-who",
                                                "{row.get(\"patient_display_name\").and_then(|value| value.as_str()).unwrap_or(\"Update\")}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !page.reviewed_nothing_for_you.is_empty() {
                            section {
                                class: "tu-updates-group",
                                aria_label: "Reviewed, nothing for you",
                                "data-updates-group": "reviewed-nothing-for-you",
                                div { class: "tu-updates-group-bar",
                                    div { class: "tu-updates-group-title",
                                        h2 { class: "tu-updates-heading", "Reviewed, nothing for you" }
                                        Count {
                                            value: page.reviewed_nothing_for_you.len(),
                                            words: if page.reviewed_nothing_for_you.len() == 1 {
                                                "patient"
                                            } else {
                                                "patients"
                                            },
                                        }
                                    }
                                    Button {
                                        compact: true,
                                        variant: ButtonVariant::Quiet,
                                        aria_expanded: if quiet_expanded() { "true" } else { "false" },
                                        onclick: move |_| quiet_expanded.set(!quiet_expanded()),
                                        if quiet_expanded() { "Hide" } else { "Show" }
                                    }
                                }
                                if quiet_expanded() {
                                    div { class: "today-list",
                                        for item in page.reviewed_nothing_for_you {
                                            UpdateItemRow { item, on_open: on_open_item }
                                        }
                                    }
                                }
                            }
                        }
                        if !page.earlier_days.is_empty() {
                            section {
                                class: "tu-updates-group",
                                aria_label: "Earlier days",
                                "data-updates-group": "earlier-days",
                                h2 { class: "tu-updates-heading", "Earlier days" }
                                for day in page.earlier_days {
                                    div {
                                        class: "tu-updates-day",
                                        "data-earlier-day": "{day.date}",
                                        h3 { class: "tu-updates-day-heading", "{day.label}" }
                                        div { class: "today-list",
                                            for item in day.items {
                                                UpdateItemRow { item, on_open: on_open_item }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use physician_wire::today_updates::ProducerSliceWire;

    use super::*;

    #[test]
    fn nothing_needs_you_holds_only_when_every_producer_answered_and_no_outages_exist() {
        let clean = TodayMeta::default();
        assert!(today_shows_nothing_needs_you(&[], &[], &[], &clean));

        let partial = TodayMeta {
            incomplete: true,
            failed_parts: vec![FailedProducerWire {
                producer: ProducerSliceWire::PatientReviews,
                label: "patient reviews".to_owned(),
                message: "unavailable".to_owned(),
            }],
            ..TodayMeta::default()
        };
        assert!(!today_shows_nothing_needs_you(&[], &[], &[], &partial));

        let partial_panel = TodayMeta {
            may_be_missing_someone: true,
            ..TodayMeta::default()
        };
        assert!(!today_shows_nothing_needs_you(
            &[],
            &[],
            &[],
            &partial_panel
        ));

        let outage = TodayMeta {
            practice_outages: vec![PracticeOutageWire {
                source_id: "epic".to_owned(),
                source_label: "Practice EHR".to_owned(),
                affected_workspace_count: 3,
                since: "2026-08-27T15:00:00Z".to_owned(),
                failure_detail: "FHIR endpoint timed out".to_owned(),
            }],
            ..TodayMeta::default()
        };
        assert!(!today_shows_nothing_needs_you(&[], &[], &[], &outage));

        let source_notice = physician_wire::TodayConnection {
            condition: "source_read_failed".to_owned(),
            channel: Some("spruce".to_owned()),
            source_id: Some(physician_wire::source_setup::SupportedSourceWire::Spruce),
            source_label: Some("Spruce Health".to_owned()),
            reason: Some("timeout".to_owned()),
            last_successful_read_at: None,
            recovery: Some(physician_wire::source_setup::TodayConnectionRecoveryWire::ReadFailed),
        };
        assert!(!today_shows_nothing_needs_you(
            &[],
            &[],
            &[source_notice],
            &clean
        ));
    }

    #[test]
    fn partial_failure_and_outage_sentences_name_the_affected_sources() {
        let sentence = partial_failure_sentence(
            "Today",
            &[FailedProducerWire {
                producer: ProducerSliceWire::PatientReviews,
                label: "patient reviews".to_owned(),
                message: "timeout".to_owned(),
            }],
        );
        assert_eq!(
            sentence,
            "Part of Today couldn’t load (patient reviews). Showing available items."
        );

        let outage = practice_outage_sentence(&PracticeOutageWire {
            source_id: "quest".to_owned(),
            source_label: "Quest Labs".to_owned(),
            affected_workspace_count: 4,
            since: "2026-08-27T15:00:00Z".to_owned(),
            failure_detail: "Connection refused".to_owned(),
        });
        assert_eq!(
            outage,
            "Quest Labs is unavailable across the practice (4 patients): Connection refused"
        );
    }

    #[test]
    fn citation_chip_opens_full_formatted_record_body_and_preserves_native_uri() {
        let full_body = format!(
            "Comprehensive metabolic panel\n{}\nPotassium: 6.1 mmol/L (critical high)",
            "Normal range reference line. ".repeat(20)
        );
        assert!(full_body.len() > 500);
        let citation = ItemCitationWire {
            candidate_id: "obs-bmp-1".to_owned(),
            revision: "rev-1".to_owned(),
            source_id: "regional-ehr".to_owned(),
            source_label: "Regional EHR".to_owned(),
            label: "Comprehensive metabolic panel".to_owned(),
            as_of: Some("2026-08-27".to_owned()),
            excerpt: "Short 320-char excerpt…".to_owned(),
            body: full_body.clone(),
            native_uri: Some("https://ehr.codahealth.test/chart/obs-bmp-1".to_owned()),
        };
        let chip = item_citation_to_chip(1, &citation);
        assert_eq!(chip.preview, full_body);
        assert_eq!(chip.open_label.as_deref(), Some("Open in Regional EHR"));
        assert_eq!(
            chip.open_uri.as_deref(),
            Some("https://ehr.codahealth.test/chart/obs-bmp-1")
        );
    }

    #[test]
    fn item_open_failure_uses_concise_retryable_copy() {
        let target = OpenItemTarget {
            workspace_id: "pw-1".to_owned(),
            item_id: "concern-1".to_owned(),
            label: "Serum creatinine".to_owned(),
            idempotency_key: "key-1".to_owned(),
        };
        let retryable = build_item_open_failure(target.clone(), ItemOpenFailureKind::Retryable);
        assert_eq!(retryable.message, "Couldn’t load this");
        assert!(retryable.retryable);
        assert_eq!(retryable.target, target);

        let unavailable = build_item_open_failure(target, ItemOpenFailureKind::UnavailableOnPanel);
        assert_eq!(
            unavailable.message,
            "This item is no longer available on your panel."
        );
        assert!(!unavailable.retryable);
    }
}
