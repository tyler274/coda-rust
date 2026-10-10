//! Maps the server's Today and workspace JSON onto the shipping shell.
//! The reader shows the row the physician chose. It does not fetch a citation.

use physician_wire::{
    AutomationRun, ClinicalSegment, PatientWorkspaceResponse, PracticeDayResponse, TodayConnection,
    TodayListResponse, TodayRow, VisitPreparation,
};

pub const TODAY_LOADING: &str = "Loading Today…";
pub const TODAY_UNAVAILABLE: &str = "Coda couldn’t load Today.";
pub const TODAY_STALE: &str = "Coda couldn’t refresh Today. Showing the last list.";
pub const TODAY_QUIET: &str = "Nothing needs you right now.";
pub const TODAY_QUIET_DETAIL: &str = "Coda will bring anything new here as it arrives.";
pub const WORKSPACE_CLOSED: &str = "Open a workspace to see its chats.";
pub const WORKSPACE_EMPTY: &str = "This workspace has no chats.";
pub const WORKSPACE_UNAVAILABLE: &str = "Coda couldn’t load what needs you for this patient.";
#[allow(dead_code)]
pub const READER_EMPTY: &str = "Nothing is open to read.";
pub const TRY_AGAIN: &str = "Try again";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodayLine {
    pub item_id: String,
    pub workspace_id: String,
    pub patient: String,
    pub kind: String,
    pub reason: String,
    pub did: String,
    pub waiting_since: String,
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
    pub preparation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodayView {
    Loading,
    Unavailable,
    Quiet,
    Stale(Vec<TodayLine>),
    Rows(Vec<TodayLine>),
}

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
        resume_work_item_id: row
            .resume_target
            .as_ref()
            .map(|target| target.work_item_id.clone()),
        resume_run_id: row
            .resume_target
            .as_ref()
            .map(|target| target.expected_run_id.clone()),
    }
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
    }
}

/// The patient's recorded visit, including one that has already ended.
pub fn patient_visit(day: &PracticeDayResponse, workspace_id: &str) -> Option<VisitLine> {
    day.visits
        .iter()
        .find(|visit| visit.patient_workspace_id.as_deref() == Some(workspace_id))
        .map(|visit| visit_line(day, visit))
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
                loop {
                    let ClinicalSegment::Citation {
                        state,
                        display_label,
                        source,
                        source_id,
                        handle,
                    } = &segments[index]
                    else {
                        break;
                    };
                    let number = chip_number(
                        state,
                        source.as_deref(),
                        source_id.as_deref(),
                        handle.as_deref(),
                        &mut numbers,
                        &mut next,
                    );
                    let label = display_label
                        .as_deref()
                        .map(str::trim)
                        .filter(|label| !label.is_empty())
                        .unwrap_or("Source");
                    chips.push(SourceChip {
                        index: number,
                        label: label.to_owned(),
                    });
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

fn chip_number(
    state: &str,
    source: Option<&str>,
    source_id: Option<&str>,
    handle: Option<&str>,
    numbers: &mut Vec<(String, usize)>,
    next: &mut usize,
) -> usize {
    if state == "resolved" {
        if let (Some(source), Some(source_id), Some(handle)) = (source, source_id, handle) {
            let key = format!("{source}\n{source_id}\n{handle}");
            if let Some((_, number)) = numbers.iter().find(|(existing, _)| existing == &key) {
                return *number;
            }
            let number = *next;
            numbers.push((key, number));
            *next += 1;
            return number;
        }
    }
    let number = *next;
    *next += 1;
    number
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavCommand {
    Search,
    Shortcuts,
    Next,
    Previous,
    Activate,
}

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
    let z = unix.div_euclid(86_400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let month_part = (5 * doy + 2) / 153;
    let month = if month_part < 10 {
        month_part + 3
    } else {
        month_part - 9
    };
    let year = if month <= 2 { y + 1 } else { y };
    year as i32
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

/// `current` is the latest read. `None` means that read has not answered.
/// A failed refresh keeps `previous`; a failed first read does not.
pub fn today_view(
    previous: Option<&[TodayLine]>,
    current: Option<Result<&TodayListResponse, ()>>,
) -> TodayView {
    match current {
        None => match previous {
            Some(rows) if !rows.is_empty() => TodayView::Rows(rows.to_vec()),
            _ => TodayView::Loading,
        },
        Some(Err(())) => match previous {
            Some(rows) if !rows.is_empty() => TodayView::Stale(rows.to_vec()),
            _ => TodayView::Unavailable,
        },
        Some(Ok(list)) if list.rows.is_empty() => TodayView::Quiet,
        Some(Ok(list)) => TodayView::Rows(today_lines(list)),
    }
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

pub fn chat_view(
    selected: Option<&str>,
    current: Option<Result<&PatientWorkspaceResponse, ()>>,
) -> ChatView {
    if selected.is_none() {
        return ChatView::Closed;
    }
    match current {
        None => ChatView::Pending,
        Some(Err(())) => ChatView::Unavailable,
        Some(Ok(workspace)) if workspace.chats.is_empty() => ChatView::Empty,
        Some(Ok(workspace)) => ChatView::Chats(
            workspace
                .chats
                .iter()
                .map(|chat| ChatLine {
                    id: chat.chat_id.clone(),
                    title: chat.title.clone().unwrap_or_else(|| "Untitled".to_owned()),
                })
                .collect(),
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ReaderView {
    Closed,
    Open { patient: String, reason: String },
}

#[allow(dead_code)]
pub fn reader_view(line: Option<&TodayLine>) -> ReaderView {
    match line {
        None => ReaderView::Closed,
        Some(line) => ReaderView::Open {
            patient: line.patient.clone(),
            reason: line.reason.clone(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(today_view(None, None), TodayView::Loading);
        let empty = today_json("");
        assert_eq!(today_view(None, Some(Ok(&empty))), TodayView::Quiet);
        assert_eq!(today_view(None, Some(Err(()))), TodayView::Unavailable);
        let loaded = today_json(&row());
        let lines = today_lines(&loaded);
        assert_eq!(
            today_view(None, Some(Ok(&loaded))),
            TodayView::Rows(lines.clone())
        );
        assert_eq!(
            today_view(Some(&lines), Some(Err(()))),
            TodayView::Stale(lines)
        );
    }

    #[test]
    fn workspace_chats_and_reader_use_the_open_selection() {
        assert_eq!(chat_view(None, None), ChatView::Closed);
        let empty: PatientWorkspaceResponse = serde_json::from_str(
            r#"{"patient_workspace_id":"p1","display_name":"Ada","chats":[]}"#,
        )
        .unwrap();
        assert_eq!(chat_view(Some("p1"), Some(Ok(&empty))), ChatView::Empty);
        let open: PatientWorkspaceResponse = serde_json::from_str(
            r#"{"patient_workspace_id":"p1","display_name":"Ada","chats":[{"chat_id":"c1","title":"Morning","created_at":"2026-10-09T12:00:00Z"}]}"#,
        )
        .unwrap();
        assert_eq!(
            chat_view(Some("p1"), Some(Ok(&open))),
            ChatView::Chats(vec![ChatLine {
                id: "c1".to_owned(),
                title: "Morning".to_owned(),
            }])
        );
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
        assert_eq!(reader_view(None), ReaderView::Closed);
        let line = TodayLine {
            item_id: "i1".to_owned(),
            workspace_id: "p1".to_owned(),
            patient: "Ada".to_owned(),
            kind: "concern".to_owned(),
            reason: "New note".to_owned(),
            did: "New note".to_owned(),
            waiting_since: "2026-10-09T12:00:00Z".to_owned(),
            resume_work_item_id: None,
            resume_run_id: None,
        };
        assert_eq!(
            reader_view(Some(&line)),
            ReaderView::Open {
                patient: "Ada".to_owned(),
                reason: "New note".to_owned()
            }
        );
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
        let ahead = upcoming_visits(&day);
        assert_eq!(ahead.len(), 1);
        assert_eq!(ahead[0].patient, "Mary Louis");
        assert_eq!(
            format_clock(&ahead[0].starts_at, &ahead[0].time_zone).as_deref(),
            Some("3:30 PM")
        );
        let mary = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
        let visit = patient_visit(&day, mary).unwrap();
        assert_eq!(
            visit.preparation.as_deref(),
            Some("Visit preparation failed. No new briefing was completed.")
        );
        assert_eq!(
            format_clock(&visit.starts_at, &visit.time_zone).as_deref(),
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
        let chips = pieces.iter().find_map(|piece| match piece {
            SynopsisPiece::Chips(chips) => Some(chips),
            SynopsisPiece::Text(_) => None,
        });
        let chips = chips.expect("a source chip");
        assert_eq!(chips[0].index, 1);
        assert_eq!(chips[0].label, "Mary Louis");
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
        let updates: physician_wire::UpdatesResponse =
            serde_json::from_str(include_str!("../../physician-sample/fixtures/updates.json"))
                .unwrap();
        assert!(updates.rows.is_empty());
    }
}
