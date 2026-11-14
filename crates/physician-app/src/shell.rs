//! Maps the server's Today and workspace JSON onto the shipping shell.
//! The reader shows the row the physician chose. It does not fetch a citation.

use physician_wire::{
    AutomationRun, CitationDestination, ClinicalSegment, ConnectedSource, PatientWorkspaceResponse,
    PracticeDayResponse, ReplyRevisionRun, SourcePresentation, TodayConnection, TodayListResponse,
    TodayRow, VisitPreparation, WhoamiResponse, WorkspaceReply,
};

pub const TODAY_LOADING: &str = "Loading Today…";
pub const TODAY_UNAVAILABLE: &str = "Coda couldn’t load Today.";
pub const TODAY_STALE: &str = "Coda couldn’t refresh Today. Showing the last list.";
pub const TODAY_QUIET: &str = "Nothing needs you right now.";
pub const TODAY_QUIET_DETAIL: &str = "Coda will bring anything new here as it arrives.";
pub const WORKSPACE_CLOSED: &str = "Open a workspace to see its chats.";
pub const WORKSPACE_EMPTY: &str = "This workspace has no chats.";
pub const WORKSPACE_UNAVAILABLE: &str = "Coda couldn’t load what needs you for this patient.";
pub const TRY_AGAIN: &str = "Try again";
/// Stops the chat turn Coda is still writing.
pub const STOP: &str = "Stop";
/// Resumes a turn the physician stopped. Failure still says [`TRY_AGAIN`].
pub const RESUME: &str = "Resume";
/// The line after the physician stopped an answer.
pub const STOPPED: &str = "Coda stopped this response. Your message is saved.";
/// Past work with no history dates or items on the projection.
pub const PAST_WORK_EMPTY: &str = "No past conversations or work are available.";
/// An open conversation whose snapshot has no turns.
pub const CONVERSATION_EMPTY: &str = "What would you like Coda to review?";
/// A resolved citation whose preview prose is missing.
pub const SOURCE_UNAVAILABLE: &str = "Source content is unavailable here.";
/// Copies the reply draft as shown.
pub const REPLY_COPY: &str = "Copy reply";
/// Closes a ready reply without sending it.
pub const REPLY_NO_NEEDED: &str = "No reply needed";
/// The revise field's placeholder.
pub const REPLY_TELL_CODA: &str = "Tell Coda what to change";
/// Asks the server to revise the reply draft.
pub const REPLY_ASK: &str = "Ask Coda to revise";
/// Shown after the server accepts `reply.close`.
pub const REPLY_CLOSED_NOTICE: &str = "Marked no reply needed";
/// A revision request is waiting for the worker.
pub const REPLY_REVISION_PENDING: &str = "Coda will revise this reply shortly.";
/// The worker is revising the reply.
pub const REPLY_REVISION_ACTIVE: &str = "Coda is revising this reply.";
/// The latest revision replaced the saved draft.
pub const REPLY_REVISION_COMPLETED: &str = "Coda revised this reply.";
/// The latest revision ended without changing the draft.
pub const REPLY_REVISION_FAILED: &str =
    "Coda couldn’t revise this reply. The saved draft is unchanged.";
/// A newer saved edit replaced the draft the revision started from.
pub const REPLY_REVISION_SUPERSEDED: &str =
    "A newer saved edit replaced the draft, so that revision was not applied.";
/// The revision request named a draft that has since changed.
pub const REPLY_REVISION_STALE: &str =
    "Coda couldn’t take that revision. The current draft is shown; review it and ask again.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodayLine {
    pub item_id: String,
    pub workspace_id: String,
    pub patient: String,
    pub kind: String,
    pub reason: String,
    pub did: String,
    pub waiting_since: String,
    /// Coda marked the concern as needing the physician now, not just today.
    pub now: bool,
    pub resume_work_item_id: Option<String>,
    pub resume_run_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisitLine {
    pub workspace_id: Option<String>,
    pub patient: String,
    pub label: String,
    pub starts_at: String,
    pub time_zone: String,
    /// The long preparation sentence, for the reader. The rail uses `cue`.
    pub preparation: Option<String>,
    /// The short preparation cue shown on the visit, such as `Preparation failed`.
    pub cue: Option<String>,
}

/// One read the page shows: loading, showing, showing with one quiet line and
/// Retry, or couldn't load with Retry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadView<T> {
    Loading,
    Showing(T),
    Stale(T),
    Unavailable,
}

impl<T> ReadView<T> {
    pub fn stage(&self) -> crate::status::ReadStage {
        match self {
            Self::Loading => crate::status::ReadStage::Loading,
            Self::Showing(_) => crate::status::ReadStage::Showing,
            Self::Stale(_) => crate::status::ReadStage::Stale,
            Self::Unavailable => crate::status::ReadStage::Unavailable,
        }
    }

    /// What shows: the answer, or the last one under a failed refresh.
    pub fn content(&self) -> Option<&T> {
        match self {
            Self::Showing(value) | Self::Stale(value) => Some(value),
            Self::Loading | Self::Unavailable => None,
        }
    }
}

/// `current` is the latest read. `None` means that read has not answered.
/// A failed refresh keeps `previous`; a failed first read does not.
pub fn read_view<T>(previous: Option<T>, current: Option<Result<T, ()>>) -> ReadView<T> {
    match (current, previous) {
        (Some(Ok(value)), _) => ReadView::Showing(value),
        (Some(Err(())), Some(last)) => ReadView::Stale(last),
        (Some(Err(())), None) => ReadView::Unavailable,
        (None, Some(last)) => ReadView::Showing(last),
        (None, None) => ReadView::Loading,
    }
}

/// A read's answer, kept with the question it answered. A rerun leaves the last
/// answer in place until the new one lands, so a page asking a new question (a
/// query, a patient, a conversation) has no answer yet.
pub fn answer_to<'a, Q: PartialEq, T>(asking: &Q, answer: Option<&'a (Q, T)>) -> Option<&'a T> {
    answer
        .filter(|(asked, _)| asked == asking)
        .map(|(_, answer)| answer)
}

pub type TodayView = ReadView<Vec<TodayLine>>;

pub fn today_lines(list: &TodayListResponse) -> Vec<TodayLine> {
    list.rows.iter().map(today_line).collect()
}

fn today_line(row: &TodayRow) -> TodayLine {
    TodayLine {
        item_id: row.item_id.clone(),
        workspace_id: row.patient_workspace_id.clone(),
        patient: row.patient_display_name.clone(),
        kind: row.kind.clone(),
        reason: row.reason.clone().unwrap_or_default(),
        did: row_did(row),
        waiting_since: row.waiting_since.clone(),
        now: row.urgency == "now",
        resume_work_item_id: row
            .resume_target
            .as_ref()
            .map(|target| target.work_item_id.0.clone()),
        resume_run_id: row
            .resume_target
            .as_ref()
            .map(|target| target.expected_run_id.clone()),
    }
}

/// The row's state as its pill's tone, after the React app's Today: a flagged concern is a warning,
/// a drafted reply is ready, a review that couldn't finish failed, and anything else is neutral.
pub fn row_tone(kind: &str) -> ui::Tone {
    match kind {
        "concern" => ui::Tone::Warning,
        "reply" => ui::Tone::Primary,
        "failed_review" => ui::Tone::Error,
        _ => ui::Tone::Neutral,
    }
}

/// The row's kind as its pill's leading icon.
pub fn row_icon(kind: &str) -> Option<ui::IconName> {
    Some(match kind {
        "concern" => ui::IconName::Flag,
        "reply" => ui::IconName::Reply,
        _ => ui::IconName::Alert,
    })
}

/// What the row's state pill says. A failed review uses the work kind's subject.
pub fn row_did(row: &TodayRow) -> String {
    if row.kind == "failed_review" {
        return format!("{} couldn’t finish", work_subject(row.work_kind.as_deref()));
    }
    row.reason.clone().unwrap_or_default()
}

fn work_subject(kind: Option<&str>) -> &'static str {
    match kind {
        Some("patient_initialization" | "summary_maintenance") => "Patient summary",
        Some("pre_visit_preparation") => "Visit preparation",
        Some("review") => "Review",
        Some("conversation") => "Response",
        _ => "Work",
    }
}

pub fn need_you_words(count: usize) -> &'static str {
    if count == 1 { "needs you" } else { "need you" }
}

/// One sentence per distinct connection condition, in the shipping words.
pub fn connection_lines(connections: &[TodayConnection]) -> Vec<String> {
    let mut lines = Vec::new();
    for connection in connections {
        let Some(line) = connection_sentence(connection.channel.as_deref(), &connection.condition)
        else {
            continue;
        };
        if !lines.contains(&line) {
            lines.push(line);
        }
    }
    lines
}

pub fn connection_sentence(channel: Option<&str>, condition: &str) -> Option<String> {
    let name = match channel {
        Some("gmail") => "Gmail",
        Some("spruce") => "Spruce",
        None => "a connection",
        Some(_) => return None,
    };
    let line = match condition {
        "reconnect_required" => format!("Coda can’t read {name} until it’s reconnected."),
        "blocked_by_administrator" => format!(
            "Coda can’t read {name}. Its Google administrator must allow Coda, then reconnect it."
        ),
        "temporarily_unavailable" => format!("Coda can’t reach {name} right now."),
        "disconnected" => format!("Coda isn’t reading {name}: it’s disconnected."),
        "not_yet_read" => format!("Coda hasn’t read {name} yet."),
        _ => return None,
    };
    Some(line)
}

/// Today shows the schedule unless the calendar is absent. An empty usable day still shows it.
pub fn shows_today_schedule(day: &PracticeDayResponse) -> bool {
    !matches!(day.source_state.state.as_str(), "unavailable" | "absent")
}

/// Who a panel change is for: the physician chosen while the practice still lists them, else the
/// signed-in physician. The picker shows this one, and Assign and Remove send it.
pub fn panel_target<'a>(account: &'a WhoamiResponse, chosen: &str) -> &'a str {
    account
        .practice_accounts
        .iter()
        .find(|listed| listed.physician_id == chosen)
        .map_or(&account.physician_id, |listed| &listed.physician_id)
}

/// The line under an empty schedule: none were scheduled, or none remain ahead.
pub fn schedule_empty_copy(day: &PracticeDayResponse) -> &'static str {
    if day.visits.is_empty() {
        "No visits scheduled today."
    } else {
        "No visits remaining today."
    }
}

/// Visits still ahead on a usable practice day, next first, in the server's order.
pub fn upcoming_visits(day: &PracticeDayResponse) -> Vec<VisitLine> {
    if !matches!(day.source_state.state.as_str(), "usable" | "refreshing") {
        return Vec::new();
    }
    let now = parse_unix(&day.current_time);
    day.visits
        .iter()
        .filter(|visit| {
            visit.status == "scheduled"
                && match (&visit.ends_at, now) {
                    (Some(end), Some(now)) => parse_unix(end).is_none_or(|ends| ends > now),
                    _ => true,
                }
        })
        .map(|visit| visit_line(day, visit))
        .collect()
}

fn visit_line(day: &PracticeDayResponse, visit: &physician_wire::PracticeVisit) -> VisitLine {
    VisitLine {
        workspace_id: visit.patient_workspace_id.clone(),
        patient: visit
            .patient_display_name
            .clone()
            .unwrap_or_else(|| visit.label.clone()),
        label: visit.label.clone(),
        starts_at: visit.starts_at.clone(),
        time_zone: visit
            .patient_time_zone
            .clone()
            .unwrap_or_else(|| day.time_zone.clone()),
        preparation: visit.preparation.as_ref().and_then(preparation_sentence),
        cue: visit.preparation.as_ref().and_then(preparation_cue),
    }
}

/// Visits still ahead for one patient, in the practice day's order.
pub fn patient_upcoming_visits(day: &PracticeDayResponse, workspace_id: &str) -> Vec<VisitLine> {
    upcoming_visits(day)
        .into_iter()
        .filter(|visit| visit.workspace_id.as_deref() == Some(workspace_id))
        .collect()
}

/// The short cue on a visit. Scheduled and not-yet-prepared stays quiet.
pub fn preparation_cue(preparation: &VisitPreparation) -> Option<String> {
    if matches!(preparation.state.as_str(), "scheduled" | "pending") {
        return None;
    }
    let label = if let Some(attempt) = &preparation.last_attempt {
        match attempt.outcome.as_str() {
            "retained" => "Preparation incomplete",
            "failed" => "Preparation failed",
            "cancelled" => "Preparation stopped",
            "interrupted" => "Preparation interrupted",
            _ => return None,
        }
    } else {
        match preparation.state.as_str() {
            "preparing" => "Preparing visit",
            "prepared" => "Briefing ready",
            "could_not_prepare" => "Couldn’t prepare",
            "unavailable" => "Preparation unavailable",
            _ => return None,
        }
    };
    Some(label.to_owned())
}

pub fn preparation_sentence(preparation: &VisitPreparation) -> Option<String> {
    if let Some(attempt) = &preparation.last_attempt {
        return Some(
            match attempt.outcome.as_str() {
                "retained" => "Visit preparation was incomplete. No new briefing was completed.",
                "failed" => "Visit preparation failed. No new briefing was completed.",
                "cancelled" => "Visit preparation was stopped. No new briefing was completed.",
                "interrupted" => "Visit preparation was interrupted and has not resumed.",
                _ => return None,
            }
            .to_owned(),
        );
    }
    Some(
        match preparation.state.as_str() {
            "scheduled" => "The preparation window has not opened yet.",
            "pending" => "This visit has not been prepared.",
            "could_not_prepare" => "Coda couldn’t complete the pre-visit review.",
            "unavailable" => "This appointment is not connected to an available Coda patient.",
            _ => return None,
        }
        .to_owned(),
    )
}

pub fn work_heading(run: &AutomationRun) -> Option<String> {
    if run.state != "failed" {
        return run
            .purpose
            .as_ref()
            .and_then(|purpose| purpose.headline.clone());
    }
    Some(format!(
        "{} couldn’t finish",
        work_subject(Some(run.kind.as_str()))
    ))
}

pub fn work_end_reason(cause: Option<&str>) -> Option<&'static str> {
    match cause {
        Some("conversation_moved") => {
            Some("This conversation is no longer in this patient’s record.")
        }
        Some("review_superseded") => Some("A newer review of this conversation replaced it."),
        Some("reply_settled") => Some("The reply it was for was already settled or drafted."),
        Some("unavailable") => Some("Coda can’t restart this work."),
        Some("visit_ended") => Some("The visit has ended."),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// One visible piece of a patient summary.
pub enum SynopsisPiece {
    /// Prose.
    Text(String),
    /// Numbered source chips that share one run.
    Chips(Vec<SourceChip>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A numbered source chip.
pub struct SourceChip {
    /// The visible source number.
    pub index: usize,
    /// The source title.
    pub label: String,
    /// A resolved citation opens. An unresolved one stays a static chip.
    pub open: bool,
    /// The lead's preview prose. Empty when the chip stays closed.
    pub preview: String,
    /// The preview's as-of date, as the citation recorded it.
    pub as_of: String,
    /// `Open in {chart}` or `Open original` when a native link exists.
    pub open_label: Option<String>,
    /// The native destination URI.
    pub open_uri: Option<String>,
}

/// The accessible name React gives a source chip.
pub fn chip_name(chip: &SourceChip) -> String {
    if chip.open {
        format!("Source {}: {}. Preview source.", chip.index, chip.label)
    } else {
        format!("Source {} unavailable", chip.index)
    }
}

/// Splits preview prose on a blank line, the way the React preview does.
pub fn preview_paragraphs(text: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' {
            let mut end = index;
            while end < bytes.len() && bytes[end] == b'\n' {
                end += 1;
            }
            if end - index >= 2 {
                let piece = text[start..index].trim();
                if !piece.is_empty() {
                    paragraphs.push(piece.to_owned());
                }
                start = end;
            }
            index = end;
        } else {
            index += 1;
        }
    }
    let piece = text[start..].trim();
    if !piece.is_empty() {
        paragraphs.push(piece.to_owned());
    }
    paragraphs
}

/// `Prepared by Coda · as of {as_of}` when the citation recorded a date.
pub fn prepared_line(as_of: &str) -> Option<String> {
    let as_of = as_of.trim();
    if as_of.is_empty() {
        None
    } else {
        Some(format!("Prepared by Coda · as of {as_of}"))
    }
}

/// Groups typed segments the way the React citation run does.
/// The same resolved source keeps one number. Markers in `authored_text` are not read.
pub fn synopsis_pieces(segments: &[ClinicalSegment]) -> Vec<SynopsisPiece> {
    let mut parts = Vec::new();
    let mut numbers: Vec<(String, usize)> = Vec::new();
    let mut next = 1usize;
    let mut index = 0;
    while index < segments.len() {
        match &segments[index] {
            ClinicalSegment::Text { text } => {
                parts.push(SynopsisPiece::Text(text.clone()));
                index += 1;
            }
            ClinicalSegment::Citation { .. } => {
                let mut chips = Vec::new();
                while let ClinicalSegment::Citation {
                    state,
                    display_label,
                    source,
                    source_id,
                    handle,
                    source_presentation,
                    destination,
                } = &segments[index]
                {
                    let number = chip_number(
                        state,
                        source.as_deref(),
                        source_id.as_deref(),
                        handle.as_deref(),
                        &mut numbers,
                        &mut next,
                    );
                    chips.push(source_chip(
                        number,
                        state,
                        display_label.as_deref(),
                        source_presentation.as_ref(),
                        destination.as_ref(),
                    ));
                    if index + 1 < segments.len()
                        && matches!(segments[index + 1], ClinicalSegment::Citation { .. })
                    {
                        index += 1;
                        continue;
                    }
                    if index + 2 < segments.len()
                        && matches!(segments[index + 2], ClinicalSegment::Citation { .. })
                        && matches!(
                            &segments[index + 1],
                            ClinicalSegment::Text { text } if text.chars().all(|c| c == ' ' || c == '\t')
                        )
                    {
                        index += 2;
                        continue;
                    }
                    break;
                }
                parts.push(SynopsisPiece::Chips(chips));
                index += 1;
            }
        }
    }
    parts
}

fn source_chip(
    index: usize,
    state: &str,
    display_label: Option<&str>,
    presentation: Option<&SourcePresentation>,
    destination: Option<&CitationDestination>,
) -> SourceChip {
    let label = display_label
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .unwrap_or("Source");
    let open = state == "resolved";
    let content = presentation.and_then(|presentation| presentation.content.as_ref());
    let (preview, as_of) = if open {
        (
            content
                .map(|content| content.text.clone())
                .unwrap_or_default(),
            content
                .map(|content| content.as_of.clone())
                .unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    };
    let (open_label, open_uri) = native_open(open, destination);
    SourceChip {
        index,
        label: label.to_owned(),
        open,
        preview,
        as_of,
        open_label,
        open_uri,
    }
}

fn native_open(
    open: bool,
    destination: Option<&CitationDestination>,
) -> (Option<String>, Option<String>) {
    if !open {
        return (None, None);
    }
    let Some(destination) = destination else {
        return (None, None);
    };
    if destination.kind != "source_native" {
        return (None, None);
    }
    let Some(uri) = destination
        .uri
        .as_deref()
        .map(str::trim)
        .filter(|uri| !uri.is_empty())
    else {
        return (None, None);
    };
    let label = destination
        .source_label
        .as_deref()
        .map(str::trim)
        .filter(|label| !label.is_empty())
        .map(|label| format!("Open in {label}"))
        .unwrap_or_else(|| "Open original".to_owned());
    (Some(label), Some(uri.to_owned()))
}

fn chip_number(
    state: &str,
    source: Option<&str>,
    source_id: Option<&str>,
    handle: Option<&str>,
    numbers: &mut Vec<(String, usize)>,
    next: &mut usize,
) -> usize {
    if state == "resolved"
        && let (Some(source), Some(source_id), Some(handle)) = (source, source_id, handle)
    {
        let key = format!("{source}\n{source_id}\n{handle}");
        if let Some((_, number)) = numbers.iter().find(|(existing, _)| existing == &key) {
            return *number;
        }
        let number = *next;
        numbers.push((key, number));
        *next += 1;
        return number;
    }
    let number = *next;
    *next += 1;
    number
}

/// What one server-sent frame asks the client to read again.
#[derive(Debug, PartialEq, Eq)]
pub enum EventRefetch {
    /// Nothing changed that this view shows.
    Nothing,
    /// The open patient's workspace and chat.
    OpenPatient,
    /// Every projection on screen: the server missed events (bus lag or a
    /// Redis reconnect), so any of them may be stale.
    Everything,
}

/// Classifies one frame. `resync` refetches everything once; other events
/// refetch only what belongs to the open patient.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn event_refetch(event: &str, data: &str, selected: Option<&str>) -> EventRefetch {
    if event == "resync" {
        return EventRefetch::Everything;
    }
    if workspace_event_rereads(event, data, selected) {
        EventRefetch::OpenPatient
    } else {
        EventRefetch::Nothing
    }
}

/// A workspace or conversation event for the open patient means the chat should be read again.
// Callers are wasm-gated (follow_workspace_events in lib.rs); the function is
// exercised by unit tests on every target.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn workspace_event_rereads(event: &str, data: &str, selected: Option<&str>) -> bool {
    matches!((event_patient(event, data), selected), (Some(patient), Some(selected)) if patient == selected)
}

/// The patient workspace a workspace or conversation event is about, whether
/// open or not.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub fn event_patient(event: &str, data: &str) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(data).ok()?;
    let patient = match event {
        "workspace_changed" => value.get("patient_workspace_id"),
        "physician_state_changed" => {
            if value.get("kind").and_then(|kind| kind.as_str()) != Some("conversation") {
                return None;
            }
            value
                .get("target")
                .and_then(|target| target.get("patient_workspace_id"))
        }
        _ => None,
    };
    patient.and_then(|id| id.as_str()).map(str::to_owned)
}

pub fn opens_reader(kind: &str) -> bool {
    kind == "concern" || kind == "reply"
}

pub fn format_calendar_date(day: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut parts = day.split('-');
    let Some(year) = parts.next() else {
        return day.to_owned();
    };
    let Some(month) = parts.next().and_then(|value| value.parse::<usize>().ok()) else {
        return day.to_owned();
    };
    let Some(date) = parts.next().and_then(|value| value.parse::<u32>().ok()) else {
        return day.to_owned();
    };
    let Some(name) = MONTHS.get(month.wrapping_sub(1)) else {
        return day.to_owned();
    };
    format!("{name} {date}, {year}")
}

/// The sidebar where the window can't hold it beside a page: whether it is over the page, and
/// whether closing it hands focus back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Drawer {
    Closed,
    Open,
    /// Closed from inside: focus returns to the menu button that opened it.
    Dismissed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavCommand {
    Search,
    Shortcuts,
    Next,
    Previous,
    Activate,
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "modifier keys are independent; editing, modal and item focus are not yet one focus state"
)]
pub struct KeyInput {
    pub key: String,
    pub meta: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub editing: bool,
    pub modal: bool,
    pub item_focused: bool,
}

/// The same search, move, open, and shortcut keys as the React shell.
pub fn keyboard_command(input: &KeyInput) -> Option<NavCommand> {
    if input.editing || input.modal {
        return None;
    }
    let key = input.key.to_lowercase();
    if !input.alt && !input.shift && (input.meta || input.ctrl) && key == "k" {
        return Some(NavCommand::Search);
    }
    if input.meta || input.ctrl || input.alt {
        return None;
    }
    if input.key == "/" && !input.shift {
        return Some(NavCommand::Search);
    }
    if input.key == "?" {
        return Some(NavCommand::Shortcuts);
    }
    if !input.shift && key == "j" {
        return Some(NavCommand::Next);
    }
    if !input.shift && key == "k" {
        return Some(NavCommand::Previous);
    }
    if !input.shift && input.item_focused && input.key == "Enter" {
        return Some(NavCommand::Activate);
    }
    None
}

/// How long a Today row has waited: now, then whole minutes, hours, and days.
pub fn format_waited(since: &str, now: &str) -> Option<String> {
    let waited = parse_unix(now)? - parse_unix(since)?;
    let minutes = waited.max(0) / 60;
    if minutes < 1 {
        return Some("now".to_owned());
    }
    if minutes < 60 {
        return Some(format!("{minutes} min"));
    }
    let hours = minutes / 60;
    if hours < 24 {
        return Some(format!("{hours} h"));
    }
    Some(format!("{} d", hours / 24))
}

/// A clock time in the visit's zone, such as `3:30 PM`.
pub fn format_clock(iso: &str, zone: &str) -> Option<String> {
    let unix = parse_unix(iso)?;
    let offset = zone_offset(zone, unix)?;
    let day_sec = (unix + i64::from(offset)).rem_euclid(86_400);
    let hour = (day_sec / 3600) as u32;
    let minute = ((day_sec % 3600) / 60) as u32;
    let (hour12, suffix) = match hour {
        0 => (12, "AM"),
        1..=11 => (hour, "AM"),
        12 => (12, "PM"),
        _ => (hour - 12, "PM"),
    };
    Some(format!("{hour12}:{minute:02} {suffix}"))
}

/// A time against the practice day's clock: `Today, 8:00 AM` on that day.
pub fn format_context_time(iso: &str, zone: &str, now: &str) -> Option<String> {
    let clock = format_clock(iso, zone)?;
    let day = local_day(iso, zone)?;
    let today = local_day(now, zone)?;
    let prefix = match day - today {
        0 => "Today",
        -1 => "Yesterday",
        1 => "Tomorrow",
        _ => return Some(clock),
    };
    Some(format!("{prefix}, {clock}"))
}

/// The calendar date `iso` falls on in `zone`, such as `Aug 27, 2026`.
pub fn format_local_date(iso: &str, zone: &str) -> Option<String> {
    let (year, month, day) = civil_from_days(local_day(iso, zone)?);
    Some(format_calendar_date(&format!("{year}-{month}-{day}")))
}

fn local_day(iso: &str, zone: &str) -> Option<i64> {
    let unix = parse_unix(iso)?;
    let offset = zone_offset(zone, unix)?;
    Some((unix + i64::from(offset)).div_euclid(86_400))
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// One time beside the patient summary.
pub struct SummaryTime {
    /// `Summary written`, `Care reviewed`, or `Records as of`.
    pub label: &'static str,
    /// The source instant.
    pub instant: String,
    /// The words shown, such as `Today, 8:00 AM`.
    pub words: String,
}

/// Summary written and care reviewed. A missing field adds nothing.
pub fn summary_times(workspace: &PatientWorkspaceResponse, now: &str) -> Vec<SummaryTime> {
    let zone = workspace.time_zone.as_deref().unwrap_or("");
    let mut times = Vec::new();
    if let Some(instant) = workspace
        .patient_synopsis
        .as_ref()
        .and_then(|synopsis| synopsis.authored_at.clone())
    {
        push_summary_time(&mut times, "Summary written", instant, zone, now);
    }
    if let Some(instant) = workspace.last_reviewed_at.clone() {
        push_summary_time(&mut times, "Care reviewed", instant, zone, now);
    }
    times
}

/// When the patient's records were last refreshed, beside their identity.
pub fn records_time(workspace: &PatientWorkspaceResponse, now: &str) -> Option<SummaryTime> {
    let mut times = Vec::new();
    let zone = workspace.time_zone.as_deref().unwrap_or("");
    let instant = records_as_of(&workspace.connected_sources)?;
    push_summary_time(&mut times, "Records as of", instant, zone, now);
    times.pop()
}

fn push_summary_time(
    times: &mut Vec<SummaryTime>,
    label: &'static str,
    instant: String,
    zone: &str,
    now: &str,
) {
    let Some(words) = format_context_time(&instant, zone, now) else {
        return;
    };
    times.push(SummaryTime {
        label,
        instant,
        words,
    });
}

/// The oldest last good pull among the patient's connected sources.
pub fn records_as_of(sources: &[ConnectedSource]) -> Option<String> {
    let mut oldest: Option<(i64, String)> = None;
    for source in sources {
        let instant = match source.state.state.as_str() {
            "usable" | "refreshing" => source.state.synchronized_at.clone(),
            "failed" => source
                .state
                .usable_snapshot
                .as_ref()
                .and_then(|snapshot| snapshot.synchronized_at.clone()),
            _ => None,
        };
        let Some(instant) = instant else {
            continue;
        };
        let Some(unix) = parse_unix(&instant) else {
            continue;
        };
        if oldest.as_ref().is_none_or(|(when, _)| unix < *when) {
            oldest = Some((unix, instant));
        }
    }
    oldest.map(|(_, instant)| instant)
}

fn zone_offset(zone: &str, unix: i64) -> Option<i32> {
    if zone == "America/New_York" {
        return Some(eastern_offset(unix));
    }
    None
}

fn eastern_offset(unix: i64) -> i32 {
    let year = year_of(unix);
    let start = days_from_civil(year, 3, nth_sunday(year, 3, 2)) * 86_400 + 7 * 3600;
    let end = days_from_civil(year, 11, nth_sunday(year, 11, 1)) * 86_400 + 6 * 3600;
    if unix >= start && unix < end {
        -4 * 3600
    } else {
        -5 * 3600
    }
}

fn nth_sunday(year: i32, month: u32, n: u32) -> u32 {
    let first = days_from_civil(year, month, 1);
    let weekday = (first + 4).rem_euclid(7);
    let first_sunday = if weekday == 0 {
        1
    } else {
        1 + (7 - weekday as u32)
    };
    first_sunday + (n - 1) * 7
}

fn year_of(unix: i64) -> i32 {
    civil_from_days(unix.div_euclid(86_400)).0
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let month_part = (5 * doy + 2) / 153;
    let day = doy - (153 * month_part + 2) / 5 + 1;
    let month = if month_part < 10 {
        month_part + 3
    } else {
        month_part - 9
    };
    let year = if month <= 2 { y + 1 } else { y };
    (year as i32, month as u32, day as u32)
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year } as i64;
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = (year - era * 400) as u64;
    let month = month as i64;
    let day_of_year =
        (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day as i64 - 1;
    let day_of_era = yoe as i64 * 365 + yoe as i64 / 4 - yoe as i64 / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn parse_unix(value: &str) -> Option<i64> {
    let (date, rest) = value.split_once('T')?;
    let mut dates = date.split('-');
    let year: i32 = dates.next()?.parse().ok()?;
    let month: u32 = dates.next()?.parse().ok()?;
    let day: u32 = dates.next()?.parse().ok()?;
    let (clock, offset) = split_offset(rest)?;
    let clock = clock.split('.').next()?;
    let mut times = clock.split(':');
    let hour: i64 = times.next()?.parse().ok()?;
    let minute: i64 = times.next()?.parse().ok()?;
    let second: i64 = times.next().unwrap_or("0").parse().ok()?;
    let mut unix = days_from_civil(year, month, day) * 86_400 + hour * 3600 + minute * 60 + second;
    if let Some(offset) = offset {
        unix -= offset;
    }
    Some(unix)
}

fn split_offset(rest: &str) -> Option<(&str, Option<i64>)> {
    if rest.ends_with('Z') {
        return Some((rest.trim_end_matches('Z'), None));
    }
    let index = rest.rfind(['+', '-'])?;
    let (clock, offset) = rest.split_at(index);
    let sign = if offset.starts_with('-') { -1 } else { 1 };
    let digits = &offset[1..];
    let (hours, minutes) = digits.split_once(':').unwrap_or((digits, "0"));
    let seconds = hours.parse::<i64>().ok()? * 3600 + minutes.parse::<i64>().ok()? * 60;
    Some((clock, Some(sign * seconds)))
}

/// Today's read; an empty answer is the quiet list.
pub fn today_view(
    previous: Option<&[TodayLine]>,
    current: Option<Result<&TodayListResponse, ()>>,
) -> TodayView {
    read_view(
        previous.map(<[TodayLine]>::to_vec),
        current.map(|answer| answer.map(today_lines)),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatView {
    Closed,
    Pending,
    Unavailable,
    Empty,
    Chats(Vec<ChatLine>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatLine {
    pub id: String,
    pub title: String,
}

/// The workspace's conversations. `opened` is the conversation the page has
/// open; one the workspace does not list yet (opened from the address, or just
/// started elsewhere) is shown last, so its messages and composer appear.
pub fn chat_view(
    selected: Option<&str>,
    current: Option<Result<&PatientWorkspaceResponse, ()>>,
    opened: Option<&str>,
) -> ChatView {
    if selected.is_none() {
        return ChatView::Closed;
    }
    match current {
        None => ChatView::Pending,
        Some(Err(())) => ChatView::Unavailable,
        Some(Ok(workspace)) => {
            let mut lines: Vec<ChatLine> = workspace
                .chats
                .iter()
                .map(|chat| ChatLine {
                    id: chat.chat_id.clone(),
                    title: chat.title.clone().unwrap_or_else(|| "Untitled".to_owned()),
                })
                .collect();
            if let Some(id) = opened.filter(|id| lines.iter().all(|line| line.id != *id)) {
                lines.push(ChatLine {
                    id: id.to_owned(),
                    title: "Untitled".to_owned(),
                });
            }
            if lines.is_empty() {
                ChatView::Empty
            } else {
                ChatView::Chats(lines)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Clean clinic-demographic fields shown for a same-clinic patient not on the physician's panel.
pub struct DemographicCardView {
    /// Patient display name.
    pub display_name: String,
    /// Formatted date of birth, such as `Mar 18, 1972`.
    pub birth_date: Option<String>,
    /// Formatted age in years, such as `54 years`.
    pub age: Option<String>,
    /// Capitalized sex or gender value.
    pub sex_or_gender: Option<String>,
    /// Primary medical record number.
    pub mrn: Option<String>,
    /// Capitalized record status, such as `Active`.
    pub record_state: Option<String>,
}

/// Extracts the clinic-demographic card fields from a workspace response.
pub fn demographic_card_view(workspace: &PatientWorkspaceResponse) -> DemographicCardView {
    let identity = workspace.identity.as_ref();
    let display_name = if !workspace.display_name.trim().is_empty() {
        workspace.display_name.clone()
    } else {
        identity
            .and_then(|id| id.display_name.clone())
            .unwrap_or_default()
    };
    let birth_date = identity
        .and_then(|id| id.birth_date.as_deref())
        .map(format_calendar_date);
    let age = identity
        .and_then(|id| id.age_years)
        .map(|years| format!("{years} years"));
    let sex_or_gender = identity
        .and_then(|id| id.sex_or_gender.as_ref())
        .and_then(|gender| gender.value.as_deref())
        .map(capitalize_first)
        .filter(|value| !value.is_empty());
    let mrn = identity.and_then(|id| {
        id.medical_record_numbers
            .iter()
            .find_map(|number| number.value.clone())
    });
    let record_state = identity
        .and_then(|id| id.state.as_deref())
        .map(capitalize_first)
        .filter(|value| !value.is_empty());
    DemographicCardView {
        display_name,
        birth_date,
        age,
        sex_or_gender,
        mrn,
        record_state,
    }
}

fn capitalize_first(value: &str) -> String {
    let mut chars = value.trim().chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Past work is offered when the workspace says it has some and the physician can read the chart.
/// The recording carries `has_past_work` and no history dates or items, so the screen uses [`PAST_WORK_EMPTY`].
pub fn past_work_copy(workspace: &PatientWorkspaceResponse) -> Option<&'static str> {
    if workspace.has_past_work && !workspace.is_clinic_demographic() {
        Some(PAST_WORK_EMPTY)
    } else {
        None
    }
}

/// What a ready reply lets the physician do with the draft already on the projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplyReviewAction {
    /// Copy the draft text.
    Copy,
    /// Send `reply.close`.
    NoReplyNeeded,
    /// Send `reply.revise` with the physician's instruction.
    Revise,
}

/// Actions for a reply the projection already lists.
/// A recorded ready draft can be copied or closed. Only a draft a reply owner
/// holds at an exact revision can be sent back for revision, and not while a
/// revision is still pending or running; that owner has no close, send or
/// other lifecycle action, so none is offered.
/// A superseded or closed reply has no actions: the recording has no update, channel link, or reopen payload.
pub fn reply_review_actions(reply: &WorkspaceReply) -> Vec<ReplyReviewAction> {
    if reply.state != "ready" {
        return Vec::new();
    }
    let owned = reply.draft_revision.is_some();
    if reply.draft_text.trim().is_empty() {
        return if owned {
            Vec::new()
        } else {
            vec![ReplyReviewAction::NoReplyNeeded]
        };
    }
    if !owned {
        return vec![ReplyReviewAction::Copy, ReplyReviewAction::NoReplyNeeded];
    }
    let running = reply
        .revision_run
        .as_ref()
        .is_some_and(ReplyRevisionRun::running);
    if running {
        vec![ReplyReviewAction::Copy]
    } else {
        vec![ReplyReviewAction::Copy, ReplyReviewAction::Revise]
    }
}

/// What a reply's latest revision run says beside the draft, if anything.
pub fn reply_revision_words(reply: &WorkspaceReply) -> Option<&'static str> {
    let run = reply.revision_run.as_ref()?;
    Some(match run.status.as_str() {
        "pending" => REPLY_REVISION_PENDING,
        "active" => REPLY_REVISION_ACTIVE,
        "completed" => REPLY_REVISION_COMPLETED,
        "failed" => REPLY_REVISION_FAILED,
        "cancelled" => REPLY_REVISION_SUPERSEDED,
        "held" => "Revision paused. Your saved draft is unchanged.",
        _ => return None,
    })
}

/// The channel name on a reply card. Unknown channels keep the server's word, capitalized.
pub fn reply_channel_label(channel: &str) -> String {
    match channel {
        "spruce" => "Spruce".to_owned(),
        // The practice's email is its Gmail connection, and Connections calls it that.
        "email" => "Gmail".to_owned(),
        other => capitalize_first(other),
    }
}

/// A reply's urgency in words, never the raw tier.
pub fn reply_urgency_words(urgency: &str) -> String {
    match urgency {
        "urgent_ed" => "Urgent, ED".to_owned(),
        "routine" => "Routine".to_owned(),
        other => capitalize_first(&other.replace('_', " ")),
    }
}

/// The citation inspection the reader shows from the preview already on the chip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationInspection {
    /// `Source {n}`.
    pub eyebrow: String,
    /// The source title.
    pub title: String,
    /// Preview paragraphs. Empty when the citation recorded none.
    pub paragraphs: Vec<String>,
    /// `Prepared by Coda · as of {date}` when the preview has a date.
    pub provenance: Option<String>,
    /// Set when a resolved citation has no preview prose.
    pub unavailable: Option<&'static str>,
    /// `Open in {chart}` or `Open original`.
    pub open_label: Option<String>,
    /// The native destination URI.
    pub open_uri: Option<String>,
}

/// Builds the inspection surface from a chip. It does not fetch the record.
pub fn citation_inspection(chip: &SourceChip) -> CitationInspection {
    let paragraphs = preview_paragraphs(&chip.preview);
    let unavailable = if chip.open && paragraphs.is_empty() {
        Some(SOURCE_UNAVAILABLE)
    } else {
        None
    };
    CitationInspection {
        eyebrow: format!("Source {}", chip.index),
        title: chip.label.clone(),
        paragraphs,
        provenance: if unavailable.is_some() {
            None
        } else {
            prepared_line(&chip.as_of)
        },
        open_label: chip.open_label.clone(),
        open_uri: chip.open_uri.clone(),
        unavailable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn admin_with_colleague() -> WhoamiResponse {
        serde_json::from_value(serde_json::json!({
            "physician_id": "phys-admin",
            "practice_id": "practice",
            "display_name": "Dr. Admin",
            "admin": true,
            "practice_accounts": [
                { "physician_id": "phys-admin", "display_name": "Dr. Admin" },
                { "physician_id": "phys-colleague", "display_name": "Dr. Colleague" },
            ],
        }))
        .unwrap()
    }

    #[test]
    fn a_panel_change_is_for_the_physician_chosen_while_the_practice_lists_them() {
        let account = admin_with_colleague();
        assert_eq!(panel_target(&account, "phys-colleague"), "phys-colleague");
    }

    #[test]
    fn a_panel_change_is_never_for_a_physician_the_practice_no_longer_lists() {
        let account = admin_with_colleague();
        assert_eq!(panel_target(&account, "phys-gone"), "phys-admin");
        assert_eq!(panel_target(&account, ""), "phys-admin");
    }

    fn today_json(rows: &str) -> TodayListResponse {
        serde_json::from_str(&format!(
            r#"{{"date":"2026-10-09","rows":[{rows}],"as_of":1}}"#
        ))
        .unwrap()
    }

    fn row() -> String {
        r#"{"patient_workspace_id":"p1","patient_display_name":"Ada","item_id":"i1","version_id":"v1","waiting_since":"2026-10-09T12:00:00Z","urgency":"today","kind":"concern","reason":"New note"}"#.to_owned()
    }

    #[test]
    fn today_covers_loading_empty_error_stale_and_loaded() {
        assert_eq!(today_view(None, None), ReadView::Loading);
        let empty = today_json("");
        assert_eq!(
            today_view(None, Some(Ok(&empty))),
            ReadView::Showing(Vec::new())
        );
        assert_eq!(today_view(None, Some(Err(()))), ReadView::Unavailable);
        let loaded = today_json(&row());
        let lines = today_lines(&loaded);
        assert_eq!(
            today_view(None, Some(Ok(&loaded))),
            ReadView::Showing(lines.clone())
        );
        assert_eq!(
            today_view(Some(&lines), Some(Err(()))),
            ReadView::Stale(lines)
        );
    }

    #[test]
    fn an_answer_counts_only_for_the_question_it_answered() {
        let answered = ("Mary".to_owned(), Ok::<_, ()>(vec!["Mary Louis"]));
        assert_eq!(
            answer_to(&"Mary".to_owned(), Some(&answered)),
            Some(&Ok(vec!["Mary Louis"]))
        );
        assert_eq!(answer_to(&"Tanvir".to_owned(), Some(&answered)), None);
        assert_eq!(
            answer_to::<String, Result<Vec<&str>, ()>>(&"Mary".to_owned(), None),
            None
        );
    }

    #[test]
    fn a_row_says_when_coda_marked_it_now() {
        let today = today_json(&row());
        assert!(!today_lines(&today)[0].now);
        let now = today_json(&row().replace(r#""urgency":"today""#, r#""urgency":"now""#));
        assert!(today_lines(&now)[0].now);
    }

    #[test]
    fn workspace_chats_and_reader_use_the_open_selection() {
        assert_eq!(chat_view(None, None, None), ChatView::Closed);
        let empty: PatientWorkspaceResponse = serde_json::from_str(
            r#"{"patient_workspace_id":"p1","display_name":"Ada","chats":[]}"#,
        )
        .unwrap();
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&empty)), None),
            ChatView::Empty
        );
        let open: PatientWorkspaceResponse = serde_json::from_str(
            r#"{"patient_workspace_id":"p1","display_name":"Ada","chats":[{"chat_id":"c1","title":"Morning","created_at":"2026-10-09T12:00:00Z"}]}"#,
        )
        .unwrap();
        let morning = ChatLine {
            id: "c1".to_owned(),
            title: "Morning".to_owned(),
        };
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&open)), None),
            ChatView::Chats(vec![morning.clone()])
        );
        // A listed conversation is not repeated when it is the open one.
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&open)), Some("c1")),
            ChatView::Chats(vec![morning.clone()])
        );
        // A conversation the workspace does not list yet still opens.
        let started = ChatLine {
            id: "c9".to_owned(),
            title: "Untitled".to_owned(),
        };
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&empty)), Some("c9")),
            ChatView::Chats(vec![started.clone()])
        );
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&open)), Some("c9")),
            ChatView::Chats(vec![morning, started])
        );
        assert_eq!(chat_view(Some("p1"), None, Some("c9")), ChatView::Pending);
        assert_eq!(chat_view(None, None, Some("c9")), ChatView::Closed);
        assert_eq!(
            connection_sentence(Some("gmail"), "not_yet_read").as_deref(),
            Some("Coda hasn’t read Gmail yet.")
        );
        assert_eq!(
            format_waited("2026-08-27T16:58:09.846Z", "2026-08-27T19:32:45.505Z").as_deref(),
            Some("2 h")
        );
        assert_eq!(
            format_clock("2026-08-27T19:30:00.000Z", "America/New_York").as_deref(),
            Some("3:30 PM")
        );
        assert!(opens_reader("concern"));
        assert!(!opens_reader("failed_review"));
        assert_eq!(
            keyboard_command(&KeyInput {
                key: "/".to_owned(),
                meta: false,
                ctrl: false,
                alt: false,
                shift: false,
                editing: false,
                modal: false,
                item_focused: false,
            }),
            Some(NavCommand::Search)
        );
        assert_eq!(
            keyboard_command(&KeyInput {
                key: "j".to_owned(),
                meta: false,
                ctrl: false,
                alt: false,
                shift: false,
                editing: true,
                modal: false,
                item_focused: false,
            }),
            None
        );
    }

    #[test]
    fn recorded_practice_keeps_synopsis_search_and_an_ended_visit() {
        let today: TodayListResponse =
            serde_json::from_str(include_str!("../../physician-sample/fixtures/today.json"))
                .unwrap();
        assert!(today.rows.is_empty());
        assert_eq!(connection_lines(&today.connections).len(), 2);
        let day: PracticeDayResponse = serde_json::from_str(include_str!(
            "../../physician-sample/fixtures/practice-day.json"
        ))
        .unwrap();
        assert!(shows_today_schedule(&day));
        let ahead = upcoming_visits(&day);
        assert_eq!(ahead.len(), 1);
        assert_eq!(ahead[0].patient, "Mary Louis");
        assert_eq!(
            format_clock(&ahead[0].starts_at, &ahead[0].time_zone).as_deref(),
            Some("3:30 PM")
        );
        let mary = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
        let jake = "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";
        assert!(patient_upcoming_visits(&day, jake).is_empty());
        let ahead_for_mary = patient_upcoming_visits(&day, mary);
        assert_eq!(ahead_for_mary.len(), 1);
        assert_eq!(ahead_for_mary[0].label, "Follow-up visit");
        assert_eq!(ahead_for_mary[0].cue.as_deref(), Some("Preparation failed"));
        assert_eq!(
            ahead_for_mary[0].preparation.as_deref(),
            Some("Visit preparation failed. No new briefing was completed.")
        );
        assert_eq!(
            format_clock(&ahead_for_mary[0].starts_at, &ahead_for_mary[0].time_zone).as_deref(),
            Some("3:30 PM")
        );
        let workspace: PatientWorkspaceResponse = serde_json::from_str(include_str!(
            "../../physician-sample/fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json"
        ))
        .unwrap();
        let synopsis = workspace.patient_synopsis.as_ref().unwrap();
        assert!(
            synopsis
                .authored_text
                .as_deref()
                .unwrap()
                .contains("Mary Louis")
        );
        let pieces = synopsis_pieces(&synopsis.segments);
        let chips: Vec<_> = pieces
            .iter()
            .filter_map(|piece| match piece {
                SynopsisPiece::Chips(chips) => Some(chips.as_slice()),
                SynopsisPiece::Text(_) => None,
            })
            .flatten()
            .collect();
        let chart = chips.iter().find(|chip| chip.index == 1).expect("chip 1");
        assert!(chart.open);
        assert_eq!(chart.label, "Mary Louis");
        assert!(chart.preview.contains("Date of birth: 1972-03-18"));
        assert_eq!(chart.as_of, "June 23, 2026");
        assert_eq!(
            prepared_line(&chart.as_of).as_deref(),
            Some("Prepared by Coda · as of June 23, 2026")
        );
        let discharge = chips.iter().find(|chip| chip.index == 2).expect("chip 2");
        assert_eq!(discharge.label, "Discharge Summary");
        assert_eq!(discharge.as_of, "April 13, 2026");
        assert_eq!(discharge.open_label.as_deref(), Some("Open in Demo chart"));
        assert!(
            discharge
                .open_uri
                .as_deref()
                .is_some_and(|uri| uri.contains("DocumentReference"))
        );
        assert_eq!(
            chip_name(discharge),
            "Source 2: Discharge Summary. Preview source."
        );
        assert_eq!(
            preview_paragraphs("First paragraph.\n\nSecond paragraph."),
            vec![
                "First paragraph.".to_owned(),
                "Second paragraph.".to_owned()
            ]
        );
        let closed = synopsis_pieces(&[ClinicalSegment::Citation {
            state: "unavailable".to_owned(),
            display_label: Some("Missing note".to_owned()),
            source: Some("demo".to_owned()),
            source_id: Some("doc".to_owned()),
            handle: Some("h".to_owned()),
            source_presentation: Some(SourcePresentation {
                content: Some(physician_wire::SourcePreviewContent {
                    text: "This stays closed.".to_owned(),
                    as_of: "April 13, 2026".to_owned(),
                }),
            }),
            destination: Some(CitationDestination {
                kind: "source_native".to_owned(),
                source_label: Some("Demo chart".to_owned()),
                uri: Some("/local-ehr/closed".to_owned()),
            }),
        }]);
        let closed_chip = closed.iter().find_map(|piece| match piece {
            SynopsisPiece::Chips(chips) => chips.first(),
            SynopsisPiece::Text(_) => None,
        });
        let closed_chip = closed_chip.expect("an unresolved chip");
        assert!(!closed_chip.open);
        assert!(closed_chip.preview.is_empty());
        assert!(closed_chip.open_uri.is_none());
        assert_eq!(chip_name(closed_chip), "Source 1 unavailable");
        let unlabeled = source_chip(
            3,
            "resolved",
            Some("Lab"),
            None,
            Some(&CitationDestination {
                kind: "source_native".to_owned(),
                source_label: None,
                uri: Some("/local-ehr/lab".to_owned()),
            }),
        );
        assert_eq!(unlabeled.open_label.as_deref(), Some("Open original"));
        assert!(workspace_event_rereads(
            "workspace_changed",
            r#"{"patient_workspace_id":"pw-mary","cursor":2,"practice_day":true}"#,
            Some("pw-mary"),
        ));
        assert!(!workspace_event_rereads(
            "workspace_changed",
            r#"{"patient_workspace_id":"pw-other","cursor":2,"practice_day":false}"#,
            Some("pw-mary"),
        ));
        assert!(workspace_event_rereads(
            "physician_state_changed",
            r#"{"kind":"conversation","target":{"patient_workspace_id":"pw-mary"}}"#,
            Some("pw-mary"),
        ));
        assert!(!workspace_event_rereads(
            "physician_state_changed",
            r#"{"kind":"draft","target":{"patient_workspace_id":"pw-mary","chat_id":"c1"},"revision":1}"#,
            Some("pw-mary"),
        ));
        // A server-side gap rereads everything, even with no patient open.
        assert_eq!(
            event_refetch("resync", "{}", None),
            EventRefetch::Everything
        );
        assert_eq!(
            event_refetch("resync", "{}", Some("pw-mary")),
            EventRefetch::Everything
        );
        assert_eq!(
            event_refetch(
                "workspace_changed",
                r#"{"patient_workspace_id":"pw-mary"}"#,
                Some("pw-mary")
            ),
            EventRefetch::OpenPatient
        );
        assert_eq!(
            event_refetch(
                "workspace_changed",
                r#"{"patient_workspace_id":"pw-other"}"#,
                Some("pw-mary")
            ),
            EventRefetch::Nothing
        );
        assert!(pieces.iter().all(|piece| match piece {
            SynopsisPiece::Text(text) => !text.contains("[[patient_source"),
            SynopsisPiece::Chips(_) => true,
        }));
        assert_eq!(
            workspace
                .identity
                .as_ref()
                .and_then(|identity| identity.age_years),
            Some(54)
        );
        assert_eq!(
            workspace
                .identity
                .as_ref()
                .and_then(|identity| identity.medical_record_numbers.first())
                .and_then(|number| number.value.as_deref()),
            Some("00000005")
        );
        assert_eq!(
            work_heading(workspace.automation_run.as_ref().unwrap()).as_deref(),
            Some("Visit preparation couldn’t finish")
        );
        assert_eq!(
            work_end_reason(
                workspace
                    .automation_run
                    .as_ref()
                    .unwrap()
                    .ended_cause
                    .as_deref()
            ),
            Some("The visit has ended.")
        );
        let roster: physician_wire::PatientSearchResponse =
            serde_json::from_str(include_str!("../../physician-sample/fixtures/roster.json"))
                .unwrap();
        assert!(roster.patients.iter().any(|patient| {
            patient.patient_display_name == "Mary Louis"
                && patient.birth_date.as_deref() == Some("1972-03-18")
        }));
        assert_eq!(format_calendar_date("1972-03-18"), "Mar 18, 1972");
        for (instant, date) in [
            ("2026-08-27T16:00:00.000Z", "Aug 27, 2026"),
            ("2026-08-28T02:00:00.000Z", "Aug 27, 2026"),
            ("2028-02-29T17:00:00.000Z", "Feb 29, 2028"),
            ("2027-01-01T04:59:00.000Z", "Dec 31, 2026"),
        ] {
            assert_eq!(
                format_local_date(instant, "America/New_York").as_deref(),
                Some(date)
            );
        }
        assert_eq!(format_local_date("", "America/New_York"), None);
        let times = summary_times(&workspace, &day.current_time);
        assert_eq!(
            times
                .iter()
                .map(|time| (time.label, time.words.as_str()))
                .collect::<Vec<_>>(),
            vec![("Summary written", "Today, 8:00 AM")]
        );
        assert_eq!(
            records_time(&workspace, &day.current_time)
                .map(|time| time.words)
                .as_deref(),
            Some("Today, 12:57 PM")
        );
        let mut reviewed = workspace.clone();
        reviewed.last_reviewed_at = Some("2026-08-26T16:00:00.000Z".to_owned());
        assert!(
            summary_times(&reviewed, &day.current_time)
                .iter()
                .any(|time| {
                    time.label == "Care reviewed" && time.words == "Yesterday, 12:00 PM"
                })
        );
        let older = records_as_of(&[
            physician_wire::ConnectedSource {
                state: physician_wire::SourceConnectionState {
                    state: "usable".to_owned(),
                    synchronized_at: Some("2026-08-27T16:57:53.508Z".to_owned()),
                    usable_snapshot: None,
                },
            },
            physician_wire::ConnectedSource {
                state: physician_wire::SourceConnectionState {
                    state: "failed".to_owned(),
                    synchronized_at: Some("2026-08-27T18:00:00.000Z".to_owned()),
                    usable_snapshot: Some(physician_wire::SourceSnapshot {
                        synchronized_at: Some("2026-08-27T12:00:00.000Z".to_owned()),
                    }),
                },
            },
        ]);
        assert_eq!(older.as_deref(), Some("2026-08-27T12:00:00.000Z"));
        let updates: physician_wire::UpdatesResponse =
            serde_json::from_str(include_str!("../../physician-sample/fixtures/updates.json"))
                .unwrap();
        assert!(updates.rows.is_empty());
    }

    #[test]
    fn a_usable_day_with_no_visits_still_shows_the_schedule() {
        let day: PracticeDayResponse = serde_json::from_str(
            r#"{"current_time":"2026-08-30T11:00:00.000Z","time_zone":"America/New_York","source_state":{"state":"usable"},"visits":[]}"#,
        )
        .unwrap();
        assert!(shows_today_schedule(&day));
        assert!(upcoming_visits(&day).is_empty());
        assert_eq!(schedule_empty_copy(&day), "No visits scheduled today.");
        let gone: PracticeDayResponse = serde_json::from_str(
            r#"{"current_time":"2026-08-30T11:00:00.000Z","time_zone":"America/New_York","source_state":{"state":"unavailable"},"visits":[]}"#,
        )
        .unwrap();
        assert!(!shows_today_schedule(&gone));
    }

    #[test]
    fn clinic_demographic_workspace_extracts_card_fields_without_clinical_sections() {
        let workspace: PatientWorkspaceResponse = serde_json::from_str(
            r#"{
                "patient_workspace_id": "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
                "display_name": "Jake Test",
                "membership": "clinic_demographic",
                "identity": {
                    "display_name": "Jake Test",
                    "birth_date": "1958-11-04",
                    "age_years": 67,
                    "sex_or_gender": { "value": "male" },
                    "medical_record_numbers": [{ "value": "00000012" }],
                    "state": "active"
                },
                "connected_sources": [],
                "active_follow_ups": [],
                "chats": []
            }"#,
        )
        .unwrap();
        assert!(workspace.is_clinic_demographic());
        assert!(workspace.patient_synopsis.is_none());
        let card = demographic_card_view(&workspace);
        assert_eq!(card.display_name, "Jake Test");
        assert_eq!(card.birth_date.as_deref(), Some("Nov 4, 1958"));
        assert_eq!(card.age.as_deref(), Some("67 years"));
        assert_eq!(card.sex_or_gender.as_deref(), Some("Male"));
        assert_eq!(card.mrn.as_deref(), Some("00000012"));
        assert_eq!(card.record_state.as_deref(), Some("Active"));
        assert_eq!(past_work_copy(&workspace), None);
    }

    #[test]
    fn recorded_past_work_is_the_empty_history_screen() {
        let mary: PatientWorkspaceResponse = serde_json::from_str(include_str!(
            "../../physician-sample/fixtures/workspaces/pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e.json"
        ))
        .unwrap();
        assert!(mary.has_past_work);
        assert_eq!(past_work_copy(&mary), Some(PAST_WORK_EMPTY));
        let synopsis = mary.patient_synopsis.as_ref().unwrap();
        let chips: Vec<_> = synopsis_pieces(&synopsis.segments)
            .into_iter()
            .filter_map(|piece| match piece {
                SynopsisPiece::Chips(chips) => Some(chips),
                SynopsisPiece::Text(_) => None,
            })
            .flatten()
            .collect();
        let discharge = chips.iter().find(|chip| chip.index == 2).expect("chip 2");
        let inspection = citation_inspection(discharge);
        assert_eq!(inspection.eyebrow, "Source 2");
        assert_eq!(inspection.title, "Discharge Summary");
        assert!(inspection.unavailable.is_none());
        assert_eq!(
            inspection.provenance.as_deref(),
            Some("Prepared by Coda · as of April 13, 2026")
        );
        assert_eq!(inspection.open_label.as_deref(), Some("Open in Demo chart"));
        let mut closed = discharge.clone();
        closed.preview.clear();
        closed.as_of.clear();
        let missing = citation_inspection(&closed);
        assert_eq!(missing.unavailable, Some(SOURCE_UNAVAILABLE));
        assert!(missing.provenance.is_none());
    }

    #[test]
    fn ready_replies_can_be_reviewed() {
        let ready: WorkspaceReply = serde_json::from_str(
            r#"{"reply_work_id":"r1","patient_workspace_id":"p1","patient_display_name":"Jake Smith","channel":"spruce","headline":"Urgent potassium","draft_text":"Go to the ED.","urgency":"urgent_ed","created_at":"2026-08-27T12:05:00Z","state":"ready"}"#,
        )
        .unwrap();
        assert_eq!(reply_channel_label(&ready.channel), "Spruce");
        // A recorded reply has no owner revision: it cannot be revised.
        assert_eq!(
            reply_review_actions(&ready),
            vec![ReplyReviewAction::Copy, ReplyReviewAction::NoReplyNeeded]
        );
        assert_eq!(reply_revision_words(&ready), None);
        // An owner-held draft revises from its exact revision and offers no
        // close/send lifecycle action that owner does not support.
        let owned = WorkspaceReply {
            draft_revision: Some(3),
            ..ready.clone()
        };
        assert_eq!(
            reply_review_actions(&owned),
            vec![ReplyReviewAction::Copy, ReplyReviewAction::Revise]
        );
        let run = |status: &str| WorkspaceReply {
            revision_run: Some(physician_wire::ReplyRevisionRun {
                resume_target: None,
                run_id: "k".into(),
                status: status.into(),
            }),
            ..owned.clone()
        };
        for status in ["pending", "active"] {
            assert_eq!(
                reply_review_actions(&run(status)),
                vec![ReplyReviewAction::Copy],
                "{status}"
            );
        }
        for status in ["completed", "failed", "cancelled", "held"] {
            assert_eq!(
                reply_review_actions(&run(status)),
                vec![ReplyReviewAction::Copy, ReplyReviewAction::Revise],
                "{status}"
            );
        }
        assert_eq!(
            reply_revision_words(&run("pending")),
            Some(REPLY_REVISION_PENDING)
        );
        assert_eq!(
            reply_revision_words(&run("active")),
            Some(REPLY_REVISION_ACTIVE)
        );
        assert_eq!(
            reply_revision_words(&run("completed")),
            Some(REPLY_REVISION_COMPLETED)
        );
        assert_eq!(
            reply_revision_words(&run("failed")),
            Some(REPLY_REVISION_FAILED)
        );
        assert_eq!(
            reply_revision_words(&run("cancelled")),
            Some(REPLY_REVISION_SUPERSEDED)
        );
        assert_eq!(reply_revision_words(&run("unknown")), None);
        let superseded = WorkspaceReply {
            state: "superseded_by_correction".to_owned(),
            draft_text: String::new(),
            superseded_reason: Some("Corrected labs".to_owned()),
            ..ready
        };
        assert!(reply_review_actions(&superseded).is_empty());
        assert_eq!(reply_channel_label("email"), "Gmail");
        assert_eq!(reply_urgency_words("urgent_ed"), "Urgent, ED");
        assert_eq!(reply_urgency_words("same_day"), "Same day");
    }
}
