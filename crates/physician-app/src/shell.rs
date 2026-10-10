//! Maps the server's Today and workspace JSON onto the shipping shell.
//! The reader shows the row the physician chose. It does not fetch a citation.

use physician_wire::{
    PatientWorkspaceResponse, PracticeDayResponse, TodayConnection, TodayListResponse, TodayRow,
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
    pub starts_at: String,
    pub time_zone: String,
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
        .map(|visit| VisitLine {
            workspace_id: visit.patient_workspace_id.clone(),
            patient: visit
                .patient_display_name
                .clone()
                .unwrap_or_else(|| visit.label.clone()),
            starts_at: visit.starts_at.clone(),
            time_zone: visit
                .patient_time_zone
                .clone()
                .unwrap_or_else(|| day.time_zone.clone()),
        })
        .collect()
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
    Chats(Vec<String>),
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
                .map(|chat| chat.title.clone().unwrap_or_else(|| "Untitled".to_owned()))
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
            ChatView::Chats(vec!["Morning".to_owned()])
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
        assert_eq!(reader_view(None), ReaderView::Closed);
        let line = TodayLine {
            item_id: "i1".to_owned(),
            workspace_id: "p1".to_owned(),
            patient: "Ada".to_owned(),
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
    }
}
