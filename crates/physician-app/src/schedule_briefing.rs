//! Schedule pane states, browser time-zone formatting, and the pre-visit briefing view.

use std::collections::BTreeMap;

use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::briefings::{
    BriefingClaimWire, BriefingDecisionWire, BriefingLimitWire, BriefingSourceStatusWire,
    BriefingSummaryItemWire, BriefingVisitStandingWire, VisitBriefingWire,
};
use physician_wire::{CitedSourceWire, PracticeDayResponse, PracticeVisit};
use ui::{
    Button, ButtonVariant, IconName, ListRow, NavigationRow, Notice, Paper, PaperSection, RailCard,
    RowGroup, StatePill, Tone,
};

use crate::shell::{
    SourceChip, TRY_AGAIN, WORKSPACE_CLOSED, civil_from_days, days_from_civil, message_pieces,
    nth_sunday, parse_unix, preparation_cue,
};
use crate::status::ReadStage;
use crate::{
    CitedMessage, EmptyLine, NavigationControl, ReadNotice, SCHEDULE_LOADING, SCHEDULE_UNAVAILABLE,
};

const DEFAULT_FALLBACK_ZONE: &str = "America/New_York";
const BRIEFING_LOADING: &str = "Loading visit briefing…";
const BRIEFING_UNAVAILABLE: &str = "Coda couldn’t load this visit briefing.";

/// Which surface hosts the schedule rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailContext {
    /// Practice-wide schedule beside Today.
    PracticeToday,
    /// Single-patient upcoming visits on the patient page.
    PatientUpcoming,
}

/// Whether a schedule row is the next active visit or a later slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextVisitSlot {
    /// First non-cancelled upcoming visit; the only row shown on narrow viewports.
    Next,
    /// Subsequent visit row shown on desktop viewports.
    Later,
}

/// Freshness of the schedule snapshot currently shown in the rail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulePaneFreshness {
    /// Schedule snapshot is current.
    Current,
    /// Schedule refresh failed; retained visits remain visible with an `"as of"` notice.
    RefreshFailed {
        /// Complete physician-facing notice line.
        copy: String,
    },
}

/// Closed preparation state of one active or cancelled visit on the schedule pane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleVisitState {
    /// Scheduled visit not yet in active preparation.
    ScheduledQuiet,
    /// Preparation background job is running or queued.
    Preparing,
    /// Briefing is saved and ready to open.
    Ready {
        /// Saved briefing identifier.
        briefing_id: String,
        /// Optional concise visit-card orientation.
        orientation: Option<String>,
    },
    /// Calendar appointment is not matched to a Coda patient workspace.
    Unavailable,
    /// Preparation failed, stopped, or ended incomplete.
    Failed {
        /// Short physician-facing failure cue.
        cue: String,
    },
    /// Appointment was cancelled; never shown as prepared, with optional historical briefing link.
    Cancelled {
        /// Saved briefing identifier kept in history for the cancelled slot, if any.
        historical_briefing_id: Option<String>,
    },
}

/// Action triggered when the physician activates a row in the schedule pane.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleVisitActivation {
    /// Opens a saved current or historical visit briefing.
    OpenBriefing {
        /// Patient workspace identifier.
        workspace_id: String,
        /// Saved briefing identifier.
        briefing_id: String,
    },
    /// Opens the patient workspace page.
    OpenPatient {
        /// Patient workspace identifier.
        workspace_id: String,
    },
    /// Unmatched calendar visit has no patient workspace to open.
    None,
}

/// Target identifying the briefing currently open in [`BriefingWorkspaceView`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBriefingTarget {
    /// Patient workspace identifier.
    pub workspace_id: String,
    /// Saved briefing identifier.
    pub briefing_id: String,
}

/// Projected schedule visit row ready to draw in [`ScheduleRailPane`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleVisitRow {
    /// Matched patient workspace identifier when connected.
    pub workspace_id: Option<String>,
    /// Patient display name (or visit label when unmatched).
    pub patient: String,
    /// Calendar visit label.
    pub label: String,
    /// Scheduled start timestamp (RFC-3339).
    pub starts_at: String,
    /// Scheduled end timestamp (RFC-3339) when supplied.
    pub ends_at: Option<String>,
    /// Closed preparation or cancellation state.
    pub state: ScheduleVisitState,
    /// Whether this row is the next active visit.
    pub slot_order: NextVisitSlot,
}

impl ScheduleVisitRow {
    /// Resolves what activating this schedule row opens.
    pub fn activation(&self) -> ScheduleVisitActivation {
        match (&self.workspace_id, &self.state) {
            (Some(workspace_id), ScheduleVisitState::Ready { briefing_id, .. }) => {
                ScheduleVisitActivation::OpenBriefing {
                    workspace_id: workspace_id.clone(),
                    briefing_id: briefing_id.clone(),
                }
            }
            (
                Some(workspace_id),
                ScheduleVisitState::Cancelled {
                    historical_briefing_id: Some(briefing_id),
                },
            ) => ScheduleVisitActivation::OpenBriefing {
                workspace_id: workspace_id.clone(),
                briefing_id: briefing_id.clone(),
            },
            (Some(workspace_id), _) => ScheduleVisitActivation::OpenPatient {
                workspace_id: workspace_id.clone(),
            },
            (None, _) => ScheduleVisitActivation::None,
        }
    }
}

/// Resolves the physician's IANA time zone from the browser's `Intl` runtime,
/// falling back to `fallback` when running outside a browser or if empty.
pub fn browser_time_zone(fallback: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let locales = js_sys::Array::new();
        let opts = js_sys::Object::new();
        let dtf = js_sys::Intl::DateTimeFormat::new(&locales, &opts);
        let resolved = dtf.resolved_options();
        if let Ok(tz_val) =
            js_sys::Reflect::get(&resolved, &wasm_bindgen::JsValue::from_str("timeZone"))
            && let Some(tz) = tz_val.as_string()
        {
            let trimmed = tz.trim();
            if !trimmed.is_empty() {
                return trimmed.to_owned();
            }
        }
    }
    let trimmed = fallback.trim();
    if trimmed.is_empty() {
        DEFAULT_FALLBACK_ZONE.to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// Formats an RFC-3339 timestamp as a 12-hour clock (`3:30 PM`) in IANA `zone`.
pub fn format_zone_clock(iso: &str, zone: &str) -> Option<String> {
    let unix = parse_unix(iso)?;
    #[cfg(target_arch = "wasm32")]
    if let Some(formatted) = wasm_format_clock(unix, zone) {
        return Some(formatted);
    }
    host_format_clock(unix, zone)
}

/// Formats a scheduled start/end timestamp pair (`3:30 PM – 4:00 PM`) in IANA `zone`.
pub fn format_zone_window(starts_at: &str, ends_at: Option<&str>, zone: &str) -> Option<String> {
    let start = format_zone_clock(starts_at, zone)?;
    let Some(end_iso) = ends_at else {
        return Some(start);
    };
    match format_zone_clock(end_iso, zone) {
        Some(end) => Some(format!("{start} – {end}")),
        None => Some(start),
    }
}

/// Formats an `"as of"` or `"Prepared"` timestamp relative to `now_iso` in IANA `zone`
/// (`Today, 8:00 AM` on the same calendar day in `zone`, else `Aug 26, 2:00 PM`).
pub fn format_zone_as_of(iso: &str, now_iso: &str, zone: &str) -> Option<String> {
    let unix = parse_unix(iso)?;
    let clock = format_zone_clock(iso, zone)?;
    let same_day = parse_unix(now_iso)
        .and_then(|now_unix| {
            let target_ymd = zone_ymd(unix, zone)?;
            let now_ymd = zone_ymd(now_unix, zone)?;
            Some(target_ymd == now_ymd)
        })
        .unwrap_or(false);
    if same_day {
        return Some(format!("Today, {clock}"));
    }
    let (_, month, day) = zone_ymd(unix, zone)?;
    Some(format!("{} {day}, {clock}", short_month(month)?))
}

#[cfg(target_arch = "wasm32")]
fn normalize_intl_spaces(raw: String) -> String {
    raw.replace(['\u{202f}', '\u{00a0}'], " ")
}

#[cfg(target_arch = "wasm32")]
fn wasm_format_clock(unix: i64, zone: &str) -> Option<String> {
    use wasm_bindgen::JsValue;

    let locales = js_sys::Array::of1(&JsValue::from_str("en-US"));
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("timeZone"),
        &JsValue::from_str(zone),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("hour"),
        &JsValue::from_str("numeric"),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("minute"),
        &JsValue::from_str("2-digit"),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("hour12"),
        &JsValue::from_bool(true),
    );

    let dtf = js_sys::Reflect::construct(
        &js_sys::Function::from(
            js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("Intl"))
                .ok()
                .and_then(|intl| {
                    js_sys::Reflect::get(&intl, &JsValue::from_str("DateTimeFormat")).ok()
                })?,
        ),
        &js_sys::Array::of2(&locales, &opts),
    )
    .ok()?;
    let format_fn =
        js_sys::Function::from(js_sys::Reflect::get(&dtf, &JsValue::from_str("format")).ok()?);
    let date = js_sys::Date::new(&JsValue::from_f64((unix as f64) * 1000.0));
    let formatted = format_fn.call1(&dtf, &date).ok()?.as_string()?;
    let cleaned = normalize_intl_spaces(formatted).trim().to_owned();
    (!cleaned.is_empty()).then_some(cleaned)
}

#[cfg(target_arch = "wasm32")]
fn wasm_zone_ymd(unix: i64, zone: &str) -> Option<(i32, u32, u32)> {
    use wasm_bindgen::JsValue;

    let locales = js_sys::Array::of1(&JsValue::from_str("en-CA"));
    let opts = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("timeZone"),
        &JsValue::from_str(zone),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("year"),
        &JsValue::from_str("numeric"),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("month"),
        &JsValue::from_str("2-digit"),
    );
    let _ = js_sys::Reflect::set(
        &opts,
        &JsValue::from_str("day"),
        &JsValue::from_str("2-digit"),
    );

    let dtf = js_sys::Reflect::construct(
        &js_sys::Function::from(
            js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("Intl"))
                .ok()
                .and_then(|intl| {
                    js_sys::Reflect::get(&intl, &JsValue::from_str("DateTimeFormat")).ok()
                })?,
        ),
        &js_sys::Array::of2(&locales, &opts),
    )
    .ok()?;
    let format_fn =
        js_sys::Function::from(js_sys::Reflect::get(&dtf, &JsValue::from_str("format")).ok()?);
    let date = js_sys::Date::new(&JsValue::from_f64((unix as f64) * 1000.0));
    let iso_date = format_fn.call1(&dtf, &date).ok()?.as_string()?;
    let mut parts = iso_date.trim().split('-');
    let year: i32 = parts.next()?.parse().ok()?;
    let month: u32 = parts.next()?.parse().ok()?;
    let day: u32 = parts.next()?.parse().ok()?;
    Some((year, month, day))
}

fn zone_ymd(unix: i64, zone: &str) -> Option<(i32, u32, u32)> {
    #[cfg(target_arch = "wasm32")]
    if let Some(ymd) = wasm_zone_ymd(unix, zone) {
        return Some(ymd);
    }
    let offset = host_zone_offset_seconds(zone, unix)?;
    let local = unix + i64::from(offset);
    Some(civil_from_days(local.div_euclid(86_400)))
}

fn host_format_clock(unix: i64, zone: &str) -> Option<String> {
    let offset = host_zone_offset_seconds(zone, unix)?;
    let local = unix + i64::from(offset);
    let day_seconds = local.rem_euclid(86_400);
    let hour24 = day_seconds / 3600;
    let minute = (day_seconds % 3600) / 60;
    let period = if hour24 >= 12 { "PM" } else { "AM" };
    let hour12 = match hour24 % 12 {
        0 => 12,
        other => other,
    };
    Some(format!("{hour12}:{minute:02} {period}"))
}

fn host_zone_offset_seconds(zone: &str, unix: i64) -> Option<i32> {
    match zone {
        "UTC" | "Etc/UTC" | "Etc/GMT" => Some(0),
        "America/New_York" => Some(us_dst_offset(unix, -5, -4)),
        "America/Chicago" => Some(us_dst_offset(unix, -6, -5)),
        "America/Denver" => Some(us_dst_offset(unix, -7, -6)),
        "America/Phoenix" => Some(-7 * 3600),
        "America/Los_Angeles" => Some(us_dst_offset(unix, -8, -7)),
        "America/Anchorage" => Some(us_dst_offset(unix, -9, -8)),
        "Pacific/Honolulu" => Some(-10 * 3600),
        "Europe/London" => Some(uk_bst_offset(unix)),
        "Asia/Tokyo" => Some(9 * 3600),
        _ => None,
    }
}

fn us_dst_offset(unix: i64, standard_hours: i32, daylight_hours: i32) -> i32 {
    let year = civil_from_days(unix.div_euclid(86_400)).0;
    // US DST begins 2:00 AM local standard time on the 2nd Sunday in March
    // and ends 2:00 AM local daylight time on the 1st Sunday in November.
    let start_utc = days_from_civil(year, 3, nth_sunday(year, 3, 2)) * 86_400
        + i64::from(2 - standard_hours) * 3600;
    let end_utc = days_from_civil(year, 11, nth_sunday(year, 11, 1)) * 86_400
        + i64::from(2 - daylight_hours) * 3600;
    if unix >= start_utc && unix < end_utc {
        daylight_hours * 3600
    } else {
        standard_hours * 3600
    }
}

fn uk_bst_offset(unix: i64) -> i32 {
    let year = civil_from_days(unix.div_euclid(86_400)).0;
    let march_last_sunday = last_sunday(year, 3);
    let oct_last_sunday = last_sunday(year, 10);
    let start_utc = days_from_civil(year, 3, march_last_sunday) * 86_400 + 3600;
    let end_utc = days_from_civil(year, 10, oct_last_sunday) * 86_400 + 3600;
    if unix >= start_utc && unix < end_utc {
        3600
    } else {
        0
    }
}

fn last_sunday(year: i32, month: u32) -> u32 {
    let next_month_days = if month == 12 {
        days_from_civil(year + 1, 1, 1)
    } else {
        days_from_civil(year, month + 1, 1)
    };
    let last_day = next_month_days - 1;
    let weekday = (last_day + 4).rem_euclid(7);
    let sunday = last_day - weekday;
    civil_from_days(sunday).2
}

fn short_month(month: u32) -> Option<&'static str> {
    Some(match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => return None,
    })
}

/// Computes whether the schedule pane shows a refresh-failed `"as of"` notice.
pub fn schedule_pane_freshness(
    day: Option<&PracticeDayResponse>,
    read_stage: ReadStage,
    browser_tz: &str,
) -> SchedulePaneFreshness {
    let backend_refresh_failed =
        day.is_some_and(|d| d.source_state.freshness.as_deref() == Some("refresh_failed"));
    let http_stale = read_stage == ReadStage::Stale;
    if !backend_refresh_failed && !http_stale {
        return SchedulePaneFreshness::Current;
    }

    let as_of_display = day.and_then(|d| {
        let raw = d
            .source_state
            .snapshot_as_of
            .as_deref()
            .or(d.source_state.synchronized_at.as_deref())?;
        format_zone_as_of(raw, &d.current_time, browser_tz)
    });
    let copy = match as_of_display {
        Some(when) => format!("Coda couldn’t refresh the schedule. Showing visits as of {when}."),
        None => "Coda couldn’t refresh the schedule. Showing the last one.".to_owned(),
    };
    SchedulePaneFreshness::RefreshFailed { copy }
}

/// Projects active upcoming and cancelled visits from `day` onto [`ScheduleVisitRow`]s.
///
/// Finished or ended visits (`fulfilled`, `noshow`, `preparation.state == "ended"`, or
/// `ends_at <= current_time`) drop off and never appear as currently prepared.
pub fn schedule_rows(
    day: &PracticeDayResponse,
    workspace_filter: Option<&str>,
) -> Vec<ScheduleVisitRow> {
    if !matches!(day.source_state.state.as_str(), "usable" | "refreshing") {
        return Vec::new();
    }
    let now_unix = parse_unix(&day.current_time);
    let mut rows: Vec<ScheduleVisitRow> = day
        .visits
        .iter()
        .filter(|visit| {
            if let Some(expected_ws) = workspace_filter
                && visit.patient_workspace_id.as_deref() != Some(expected_ws)
            {
                return false;
            }
            visit_stays_on_rail(visit, now_unix)
        })
        .map(|visit| project_visit_row(visit, NextVisitSlot::Later))
        .collect();

    if let Some(first_active) = rows
        .iter_mut()
        .find(|row| !matches!(row.state, ScheduleVisitState::Cancelled { .. }))
    {
        first_active.slot_order = NextVisitSlot::Next;
    }
    rows
}

fn visit_stays_on_rail(visit: &PracticeVisit, now_unix: Option<i64>) -> bool {
    if !matches!(visit.status.as_str(), "scheduled" | "cancelled") {
        return false;
    }
    if visit
        .preparation
        .as_ref()
        .is_some_and(|prep| prep.state == "ended")
    {
        return false;
    }
    match (&visit.ends_at, now_unix) {
        (Some(end_iso), Some(now)) => parse_unix(end_iso).is_none_or(|end_unix| end_unix > now),
        _ => true,
    }
}

fn project_visit_row(visit: &PracticeVisit, slot_order: NextVisitSlot) -> ScheduleVisitRow {
    let patient = visit
        .patient_display_name
        .clone()
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| visit.label.clone());
    let state = classify_visit_state(visit);
    ScheduleVisitRow {
        workspace_id: visit.patient_workspace_id.clone(),
        patient,
        label: visit.label.clone(),
        starts_at: visit.starts_at.clone(),
        ends_at: visit.ends_at.clone(),
        state,
        slot_order,
    }
}

fn classify_visit_state(visit: &PracticeVisit) -> ScheduleVisitState {
    if visit.status == "cancelled" {
        let historical_briefing_id = visit
            .preparation
            .as_ref()
            .and_then(|prep| {
                prep.historical_briefing_id
                    .clone()
                    .or_else(|| prep.briefing_id.clone())
            })
            .filter(|id| !id.trim().is_empty());
        return ScheduleVisitState::Cancelled {
            historical_briefing_id,
        };
    }

    let Some(prep) = &visit.preparation else {
        return if visit.patient_workspace_id.is_none() {
            ScheduleVisitState::Unavailable
        } else {
            ScheduleVisitState::ScheduledQuiet
        };
    };

    if prep.state == "cancelled" {
        return ScheduleVisitState::Cancelled {
            historical_briefing_id: prep
                .historical_briefing_id
                .clone()
                .filter(|id| !id.trim().is_empty()),
        };
    }
    if prep.state == "unavailable" || visit.patient_workspace_id.is_none() {
        return ScheduleVisitState::Unavailable;
    }
    if prep.state == "prepared"
        && let Some(briefing_id) = prep.briefing_id.clone().filter(|id| !id.trim().is_empty())
    {
        let orientation = prep
            .visit_card_orientation
            .clone()
            .map(|text| text.trim().to_owned())
            .filter(|text| !text.is_empty());
        return ScheduleVisitState::Ready {
            briefing_id,
            orientation,
        };
    }
    if prep.state == "preparing" && prep.last_attempt.is_none() {
        return ScheduleVisitState::Preparing;
    }
    if let Some(cue) = preparation_cue(prep)
        && (prep.state == "could_not_prepare" || prep.last_attempt.is_some())
    {
        return ScheduleVisitState::Failed { cue };
    }
    ScheduleVisitState::ScheduledQuiet
}

/// Schedule rail showing preparation states, browser-zone times, stale refresh notice,
/// and narrow-screen next-visit presentation.
#[component]
pub fn ScheduleRailPane(
    rows: Vec<ScheduleVisitRow>,
    context: RailContext,
    read: ReadStage,
    freshness: SchedulePaneFreshness,
    browser_tz: String,
    on_retry: EventHandler,
    empty: String,
    collapsed: bool,
    on_collapsed: EventHandler<bool>,
    focused: Option<usize>,
    on_activate: EventHandler<ScheduleVisitRow>,
) -> Element {
    let title = match context {
        RailContext::PracticeToday => "Schedule",
        RailContext::PatientUpcoming => "Upcoming visits",
    };
    let active_count = rows
        .iter()
        .filter(|row| !matches!(row.state, ScheduleVisitState::Cancelled { .. }))
        .count();
    let initial_notice_stage = match read {
        ReadStage::Loading | ReadStage::Unavailable => read,
        ReadStage::Showing | ReadStage::Stale => ReadStage::Showing,
    };

    rsx! {
        RailCard {
            class: if collapsed { "schedule-rail is-collapsed" } else { "schedule-rail" },
            aria_label: "{title}",
            icon: IconName::Calendar,
            title,
            count: active_count,
            count_words: "left",
            collapsed,
            on_collapsed: move |folded| on_collapsed.call(folded),
            expand_label: "Show {title}",
            collapse_label: "Collapse {title}",
            ReadNotice {
                stage: initial_notice_stage,
                loading: SCHEDULE_LOADING,
                stale: "",
                unavailable: SCHEDULE_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            if let SchedulePaneFreshness::RefreshFailed { copy } = freshness {
                div { class: "schedule-stale-notice",
                    Notice {
                        tone: Tone::Warning,
                        icon: Some(IconName::Alert),
                        title: copy,
                        Button {
                            compact: true,
                            onclick: move |_| on_retry.call(()),
                            "{TRY_AGAIN}"
                        }
                    }
                }
            }
            if rows.is_empty() && matches!(read, ReadStage::Showing | ReadStage::Stale) && !empty.is_empty() {
                p { class: "schedule-rail-empty", "{empty}" }
            }
            for (index, row) in rows.iter().enumerate() {
                ScheduleVisitItem {
                    key: "{index}-{row.starts_at}-{row.patient}",
                    row: row.clone(),
                    context,
                    browser_tz: browser_tz.clone(),
                    keyboard_focused: focused == Some(index),
                    on_activate,
                }
            }
        }
    }
}

#[component]
fn ScheduleVisitItem(
    row: ScheduleVisitRow,
    context: RailContext,
    browser_tz: String,
    keyboard_focused: bool,
    on_activate: EventHandler<ScheduleVisitRow>,
) -> Element {
    let mut classes = vec!["schedule-visit"];
    if row.slot_order == NextVisitSlot::Next {
        classes.push("is-next");
    }
    if matches!(row.state, ScheduleVisitState::Cancelled { .. }) {
        classes.push("is-cancelled");
    }
    if keyboard_focused {
        classes.push("is-keyboard-focus");
    }
    let row_class = classes.join(" ");
    let heading = match context {
        RailContext::PracticeToday => row.patient.clone(),
        RailContext::PatientUpcoming => row.label.clone(),
    };
    let aria_label = match &row.state {
        ScheduleVisitState::Ready { .. } => format!("Open briefing for {}", row.patient),
        ScheduleVisitState::Cancelled {
            historical_briefing_id: Some(_),
        } => format!("Open cancelled visit briefing for {}", row.patient),
        ScheduleVisitState::ScheduledQuiet
        | ScheduleVisitState::Preparing
        | ScheduleVisitState::Unavailable
        | ScheduleVisitState::Failed { .. }
        | ScheduleVisitState::Cancelled {
            historical_briefing_id: None,
        } => format!("Open {}", row.patient),
    };
    let clock = format_zone_clock(&row.starts_at, &browser_tz);

    rsx! {
        NavigationRow {
            class: "{row_class}",
            aria_label: "{aria_label}",
            onclick: {
                let row = row.clone();
                move |_| on_activate.call(row.clone())
            },
            div { class: "schedule-visit-top",
                if let Some(clock) = clock {
                    time { datetime: "{row.starts_at}", "{clock}" }
                }
                {render_visit_state_pill(&row.state)}
            }
            strong { class: "schedule-visit-who", "{heading}" }
            if context == RailContext::PracticeToday && row.label != row.patient {
                span { class: "schedule-visit-label", "{row.label}" }
            }
            match &row.state {
                ScheduleVisitState::Ready {
                    orientation: Some(orientation),
                    ..
                } => rsx! {
                    span { class: "schedule-visit-orientation", "{orientation}" }
                },
                ScheduleVisitState::Cancelled {
                    historical_briefing_id: Some(_),
                } => rsx! {
                    span { class: "schedule-visit-preparation", "Briefing kept in history" }
                },
                ScheduleVisitState::Ready {
                    orientation: None, ..
                }
                | ScheduleVisitState::ScheduledQuiet
                | ScheduleVisitState::Preparing
                | ScheduleVisitState::Unavailable
                | ScheduleVisitState::Failed { .. }
                | ScheduleVisitState::Cancelled {
                    historical_briefing_id: None,
                } => rsx! {},
            }
        }
    }
}

fn render_visit_state_pill(state: &ScheduleVisitState) -> Element {
    match state {
        ScheduleVisitState::ScheduledQuiet => rsx! {},
        ScheduleVisitState::Preparing => rsx! {
            StatePill { tone: Tone::Neutral, icon: Some(IconName::Clock), "Preparing" }
        },
        ScheduleVisitState::Ready { .. } => rsx! {
            StatePill { tone: Tone::Primary, icon: Some(IconName::Check), "Briefing ready" }
        },
        ScheduleVisitState::Unavailable => rsx! {
            StatePill { tone: Tone::Neutral, icon: None, "Preparation unavailable" }
        },
        ScheduleVisitState::Failed { cue } => rsx! {
            StatePill { tone: Tone::Error, icon: Some(IconName::Alert), "{cue}" }
        },
        ScheduleVisitState::Cancelled { .. } => rsx! {
            StatePill { tone: Tone::Neutral, icon: None, "Cancelled" }
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BriefingReadFailure {
    Closed,
    Unavailable,
}

/// Full pre-visit briefing reading view with inline citations, source freshness notices,
/// historical standing notice, and collapsible Basis and Sources sections.
#[component]
pub fn BriefingWorkspaceView(
    client: Signal<PhysicianClient>,
    target: OpenBriefingTarget,
    browser_tz: String,
    server_now: String,
    refresh_tick: u32,
    on_back_today: EventHandler,
    on_open_patient_page: EventHandler<String>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    let mut retry_attempt = use_signal(|| 0_u32);
    let mut basis_expanded = use_signal(|| false);
    let mut sources_expanded = use_signal(|| false);

    let ws_for_fetch = target.workspace_id.clone();
    let id_for_fetch = target.briefing_id.clone();
    let briefing_resource = use_resource(move || {
        let _tick = refresh_tick;
        let _retry = retry_attempt();
        let ws = ws_for_fetch.clone();
        let bid = id_for_fetch.clone();
        let api = client.read().clone();
        async move {
            api.visit_briefing(&ws, &bid)
                .await
                .map(|resp| resp.briefing)
                .map_err(|error| match error {
                    physician_client::HttpError::Status {
                        status: 403 | 404, ..
                    } => BriefingReadFailure::Closed,
                    physician_client::HttpError::Status { .. }
                    | physician_client::HttpError::Transport(_) => BriefingReadFailure::Unavailable,
                })
        }
    });

    let (stage, closed) = match briefing_resource.read().as_ref() {
        None => (ReadStage::Loading, false),
        Some(Ok(_)) => (ReadStage::Showing, false),
        Some(Err(BriefingReadFailure::Closed)) => (ReadStage::Showing, true),
        Some(Err(BriefingReadFailure::Unavailable)) => (ReadStage::Unavailable, false),
    };
    let loaded = briefing_resource
        .read()
        .as_ref()
        .and_then(|res| res.as_ref().ok())
        .cloned();

    let patient_ws = target.workspace_id.clone();
    rsx! {
        div { class: "reading-workspace briefing-workspace",
            nav { class: "briefing-nav", aria_label: "Briefing navigation",
                NavigationControl {
                    onclick: move |_| on_back_today.call(()),
                    "Back to Today"
                }
                if !closed {
                    Button {
                        compact: true,
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| on_open_patient_page.call(patient_ws.clone()),
                        if let Some(ref b) = loaded {
                            "{b.patient_display_name} · Patient page"
                        } else {
                            "Patient page"
                        }
                    }
                }
            }
            ReadNotice {
                stage,
                loading: BRIEFING_LOADING,
                stale: "",
                unavailable: BRIEFING_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry: move |_| *retry_attempt.write() += 1,
            }
            if closed {
                EmptyLine { message: WORKSPACE_CLOSED.to_owned() }
            }
            if let Some(briefing) = loaded {
                {render_briefing_paper(
                    briefing,
                    &browser_tz,
                    &server_now,
                    basis_expanded(),
                    EventHandler::new(move |next| basis_expanded.set(next)),
                    sources_expanded(),
                    EventHandler::new(move |next| sources_expanded.set(next)),
                    on_open_source,
                )}
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_briefing_paper(
    briefing: VisitBriefingWire,
    browser_tz: &str,
    server_now: &str,
    basis_open: bool,
    on_toggle_basis: EventHandler<bool>,
    sources_open: bool,
    on_toggle_sources: EventHandler<bool>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    let now_ref = if server_now.trim().is_empty() {
        briefing.prepared_at.as_str()
    } else {
        server_now
    };
    let visit_window = format_zone_window(&briefing.starts_at, Some(&briefing.ends_at), browser_tz)
        .unwrap_or_else(|| briefing.starts_at.clone());
    let starts_as_of = format_zone_as_of(&briefing.starts_at, now_ref, browser_tz)
        .unwrap_or_else(|| visit_window.clone());
    let prepared_display = format_zone_as_of(&briefing.prepared_at, now_ref, browser_tz)
        .unwrap_or_else(|| briefing.prepared_at.clone());
    let subtitle = format!(
        "{} · {} · Prepared {}",
        briefing.patient_display_name, visit_window, prepared_display
    );

    let standing_notice =
        standing_banner(&briefing.visit_standing, &starts_as_of, now_ref, browser_tz);
    let source_issues = collect_source_issue_lines(
        &briefing.source_freshness,
        &briefing.limits,
        now_ref,
        browser_tz,
    );
    let all_source_freshness =
        collect_all_source_lines(&briefing.source_freshness, now_ref, browser_tz);

    let basis_pieces = message_pieces(&briefing.basis_segments, &briefing.sources);
    let mut ordered_sources: Vec<CitedSourceWire> = briefing.sources.values().cloned().collect();
    ordered_sources.sort_by_key(|src| src.number);
    let source_count = ordered_sources.len();

    rsx! {
        div { class: "briefing-document",
            if let Some((notice_tone, notice_title, notice_detail)) = standing_notice {
                Notice {
                    tone: notice_tone,
                    icon: Some(IconName::History),
                    title: notice_title,
                    detail: Some(notice_detail),
                }
            }
            if !source_issues.is_empty() {
                Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: "Late or missing sources".to_owned(),
                    ul { class: "briefing-issue-list",
                        for line in source_issues {
                            li { "{line}" }
                        }
                    }
                }
            }
            Paper {
                title: briefing.visit_label.clone(),
                detail: Some(subtitle),
                p { class: "briefing-orientation", "{briefing.visit_card_orientation}" }

                if !briefing.trajectory.is_empty() {
                    PaperSection { heading: "Trajectory".to_owned(),
                        {render_claim_list(&briefing.trajectory, &briefing.sources, on_open_source)}
                    }
                }
                if !briefing.meaningful_changes.is_empty() {
                    PaperSection { heading: "Meaningful changes".to_owned(),
                        {render_claim_list(&briefing.meaningful_changes, &briefing.sources, on_open_source)}
                    }
                }
                if !briefing.unresolved_issues.is_empty() {
                    PaperSection { heading: "Unresolved issues".to_owned(),
                        {render_claim_list(&briefing.unresolved_issues, &briefing.sources, on_open_source)}
                    }
                }
                if !briefing.visit_decisions.is_empty() {
                    PaperSection { heading: "Decisions for this visit".to_owned(),
                        {render_decision_list(&briefing.visit_decisions, &briefing.sources, on_open_source)}
                    }
                }

                footer { class: "briefing-footer",
                    div { class: "briefing-disclosure-bar",
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            class: "briefing-basis-toggle",
                            aria_expanded: if basis_open { "true" } else { "false" },
                            onclick: move |_| on_toggle_basis.call(!basis_open),
                            "Basis and limitations"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            class: "briefing-sources-toggle",
                            aria_expanded: if sources_open { "true" } else { "false" },
                            onclick: move |_| on_toggle_sources.call(!sources_open),
                            if source_count > 0 {
                                "Sources ({source_count})"
                            } else {
                                "Sources"
                            }
                        }
                    }

                    if basis_open {
                        section { class: "briefing-disclosure-panel briefing-basis-panel",
                            aria_label: "Basis and limitations",
                            if !basis_pieces.is_empty() {
                                CitedMessage { pieces: basis_pieces, on_open_source }
                            }
                            if !all_source_freshness.is_empty() {
                                ul { class: "briefing-source-timestamps",
                                    for line in all_source_freshness {
                                        li { "{line}" }
                                    }
                                }
                            }
                        }
                    }

                    if sources_open {
                        section { class: "briefing-disclosure-panel briefing-sources-panel",
                            aria_label: "Sources",
                            if ordered_sources.is_empty() {
                                p { class: "dialog-note", "No cited source records on this briefing." }
                            } else {
                                RowGroup { class: "briefing-sources-list",
                                    for source in ordered_sources {
                                        li { key: "{source.source_key}",
                                            ListRow {
                                                icon: Some(IconName::File),
                                                title: format!("[{}] {}", source.number, source.title),
                                                detail: Some(source_row_detail(&source)),
                                                onclick: {
                                                    let chip = source_chip_from_cited(&source);
                                                    move |_| on_open_source.call(chip.clone())
                                                },
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
    }
}

fn render_claim_list(
    claims: &[BriefingClaimWire],
    sources: &BTreeMap<String, CitedSourceWire>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    rsx! {
        ul { class: "briefing-points",
            for (idx, claim) in claims.iter().enumerate() {
                li { key: "{idx}", class: "briefing-point",
                    CitedMessage {
                        pieces: message_pieces(&claim.segments, sources),
                        on_open_source,
                    }
                    if let Some(ref uncertainty) = claim.uncertainty {
                        p { class: "briefing-uncertainty",
                            StatePill { tone: Tone::Warning, icon: Some(IconName::Alert), "Uncertainty" }
                            span { "{uncertainty}" }
                        }
                    }
                }
            }
        }
    }
}

fn render_decision_list(
    decisions: &[BriefingDecisionWire],
    sources: &BTreeMap<String, CitedSourceWire>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    rsx! {
        div { class: "briefing-decisions",
            for (idx, item) in decisions.iter().enumerate() {
                article { key: "{idx}", class: "briefing-decision",
                    h4 { class: "briefing-decision-title", "{item.decision}" }
                    CitedMessage {
                        pieces: message_pieces(&item.rationale_segments, sources),
                        on_open_source,
                    }
                    if !item.options_or_questions.is_empty() {
                        ul { class: "briefing-decision-options",
                            for opt in &item.options_or_questions {
                                li { "{opt}" }
                            }
                        }
                    }
                    if let Some(ref uncertainty) = item.uncertainty {
                        p { class: "briefing-uncertainty",
                            StatePill { tone: Tone::Warning, icon: Some(IconName::Alert), "Uncertainty" }
                            span { "{uncertainty}" }
                        }
                    }
                }
            }
        }
    }
}

fn standing_banner(
    standing: &BriefingVisitStandingWire,
    original_slot_display: &str,
    now_iso: &str,
    browser_tz: &str,
) -> Option<(Tone, String, String)> {
    match standing {
        BriefingVisitStandingWire::Current => None,
        BriefingVisitStandingWire::Cancelled { .. } => Some((
            Tone::Warning,
            "This visit was cancelled.".to_owned(),
            format!("Kept in history for the originally scheduled slot ({original_slot_display})."),
        )),
        BriefingVisitStandingWire::Moved { new_starts_at, .. } => {
            let moved_to = format_zone_as_of(new_starts_at, now_iso, browser_tz)
                .unwrap_or_else(|| new_starts_at.clone());
            Some((
                Tone::Warning,
                format!("This visit moved to {moved_to}."),
                format!(
                    "Kept in history for the earlier scheduled slot ({original_slot_display})."
                ),
            ))
        }
        BriefingVisitStandingWire::Ended { .. } => Some((
            Tone::Neutral,
            "This visit has ended.".to_owned(),
            format!("Kept in history for the scheduled slot ({original_slot_display})."),
        )),
    }
}

fn collect_source_issue_lines(
    freshness: &[BriefingSourceStatusWire],
    limits: &[BriefingLimitWire],
    now_iso: &str,
    browser_tz: &str,
) -> Vec<String> {
    let mut lines = Vec::new();
    for status in freshness {
        match status {
            BriefingSourceStatusWire::Updated { .. } => {}
            BriefingSourceStatusWire::RefreshFailed {
                source_label,
                last_good_at,
                reason,
                ..
            } => {
                let mut line = match last_good_at
                    .as_deref()
                    .and_then(|iso| format_zone_as_of(iso, now_iso, browser_tz))
                {
                    Some(when) => {
                        format!("{source_label}: records as of {when}, couldn’t update")
                    }
                    None => format!("{source_label}: couldn’t update records"),
                };
                if let Some(detail) = reason.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
                    line.push_str(&format!(" ({detail})"));
                }
                lines.push(line);
            }
            BriefingSourceStatusWire::Unavailable { source_label, .. } => {
                lines.push(format!("{source_label}: records unavailable"));
            }
        }
    }
    for limit in limits {
        lines.push(format!("{}: {}", limit.kind.label(), limit.detail));
    }
    lines
}

fn collect_all_source_lines(
    freshness: &[BriefingSourceStatusWire],
    now_iso: &str,
    browser_tz: &str,
) -> Vec<String> {
    freshness
        .iter()
        .map(|status| match status {
            BriefingSourceStatusWire::Updated {
                source_label,
                last_good_at,
                ..
            } => {
                let when = format_zone_as_of(last_good_at, now_iso, browser_tz)
                    .unwrap_or_else(|| last_good_at.clone());
                format!("{source_label}: records as of {when}")
            }
            BriefingSourceStatusWire::RefreshFailed {
                source_label,
                last_good_at,
                ..
            } => match last_good_at
                .as_deref()
                .and_then(|iso| format_zone_as_of(iso, now_iso, browser_tz))
            {
                Some(when) => format!("{source_label}: records as of {when}, couldn’t update"),
                None => format!("{source_label}: couldn’t update records"),
            },
            BriefingSourceStatusWire::Unavailable { source_label, .. } => {
                format!("{source_label}: records unavailable")
            }
        })
        .collect()
}

fn source_row_detail(source: &CitedSourceWire) -> String {
    let mut parts = Vec::new();
    if !source.source_label.trim().is_empty() {
        parts.push(source.source_label.trim().to_owned());
    }
    if let Some(when) = source
        .recorded_at
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        parts.push(when.to_owned());
    }
    parts.join(" · ")
}

fn source_chip_from_cited(source: &CitedSourceWire) -> SourceChip {
    let preview = crate::shell::citation_preview_body(&source.quote, &source.body);
    let (open_label, open_uri) = match source
        .external_url
        .as_deref()
        .map(str::trim)
        .filter(|uri| !uri.is_empty())
    {
        Some(uri) => {
            let label = if source.source_label.trim().is_empty() {
                "Open original".to_owned()
            } else {
                format!("Open in {}", source.source_label.trim())
            };
            (Some(label), Some(uri.to_owned()))
        }
        None => (None, None),
    };
    SourceChip {
        index: source.number,
        label: source.title.clone(),
        open: true,
        preview,
        as_of: source.recorded_at.clone().unwrap_or_default(),
        open_label,
        open_uri,
    }
}

/// Summary notice words shown on a briefing history row for cancelled or moved visits.
pub fn history_standing_summary(
    item: &BriefingSummaryItemWire,
    now_iso: &str,
    browser_tz: &str,
) -> (Option<(&'static str, Tone)>, String) {
    let ref_now = if now_iso.trim().is_empty() {
        item.prepared_at.as_str()
    } else {
        now_iso
    };
    let slot_display = format_zone_as_of(&item.starts_at, ref_now, browser_tz)
        .unwrap_or_else(|| item.starts_at.clone());
    match &item.visit_standing {
        BriefingVisitStandingWire::Current => (
            Some(("Briefing ready", Tone::Primary)),
            format!("{slot_display} · {}", item.visit_card_orientation),
        ),
        BriefingVisitStandingWire::Cancelled { .. } => (
            Some(("Visit cancelled", Tone::Warning)),
            format!("Cancelled visit ({slot_display}) · Kept in history"),
        ),
        BriefingVisitStandingWire::Moved { new_starts_at, .. } => {
            let moved_to = format_zone_as_of(new_starts_at, ref_now, browser_tz)
                .unwrap_or_else(|| new_starts_at.clone());
            (
                Some(("Visit moved", Tone::Warning)),
                format!("Moved to {moved_to} (prepared for {slot_display}) · Kept in history"),
            )
        }
        BriefingVisitStandingWire::Ended { .. } => (
            Some(("Past visit", Tone::Neutral)),
            format!("Completed visit ({slot_display}) · Kept in history"),
        ),
    }
}

/// Patient-page section listing current ready briefings and preserved cancelled/moved briefings.
#[component]
pub fn PatientBriefingsSection(
    workspace_id: String,
    briefings: Vec<BriefingSummaryItemWire>,
    browser_tz: String,
    server_now: String,
    on_open_briefing: EventHandler<OpenBriefingTarget>,
) -> Element {
    if briefings.is_empty() {
        return rsx! {};
    }
    let (current, historical): (Vec<_>, Vec<_>) = briefings
        .into_iter()
        .partition(|item| matches!(item.visit_standing, BriefingVisitStandingWire::Current));

    rsx! {
        if !current.is_empty() {
            section { class: "patient-section patient-briefings-current",
                h2 { "Visit briefing" }
                RowGroup { class: "patient-briefing-rows",
                    for item in current {
                        {render_patient_briefing_row(
                            &workspace_id,
                            item,
                            &server_now,
                            &browser_tz,
                            on_open_briefing,
                        )}
                    }
                }
            }
        }
        if !historical.is_empty() {
            section { class: "patient-section patient-briefings-history",
                h2 { "Briefing history" }
                RowGroup { class: "patient-briefing-rows",
                    for item in historical {
                        {render_patient_briefing_row(
                            &workspace_id,
                            item,
                            &server_now,
                            &browser_tz,
                            on_open_briefing,
                        )}
                    }
                }
            }
        }
    }
}

fn render_patient_briefing_row(
    workspace_id: &str,
    item: BriefingSummaryItemWire,
    server_now: &str,
    browser_tz: &str,
    on_open_briefing: EventHandler<OpenBriefingTarget>,
) -> Element {
    let (badge, detail) = history_standing_summary(&item, server_now, browser_tz);
    let ws = workspace_id.to_owned();
    let bid = item.briefing_id.clone();
    rsx! {
        li { key: "{item.briefing_id}", class: "patient-briefing-item",
            NavigationRow {
                class: "patient-briefing-row",
                onclick: move |_| {
                    on_open_briefing.call(OpenBriefingTarget {
                        workspace_id: ws.clone(),
                        briefing_id: bid.clone(),
                    });
                },
                div { class: "patient-briefing-row-top",
                    strong { "{item.visit_label}" }
                    if let Some((words, tone)) = badge {
                        StatePill { tone, icon: None, "{words}" }
                    }
                }
                span { class: "patient-briefing-row-detail", "{detail}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_zone_clock_formats_across_iana_zones_and_dst() {
        let iso = "2026-08-27T19:30:00.000Z";
        assert_eq!(
            format_zone_clock(iso, "America/New_York").as_deref(),
            Some("3:30 PM")
        );
        assert_eq!(
            format_zone_clock(iso, "America/Chicago").as_deref(),
            Some("2:30 PM")
        );
        assert_eq!(
            format_zone_clock(iso, "America/Denver").as_deref(),
            Some("1:30 PM")
        );
        assert_eq!(
            format_zone_clock(iso, "America/Los_Angeles").as_deref(),
            Some("12:30 PM")
        );
        assert_eq!(
            format_zone_clock(iso, "Europe/London").as_deref(),
            Some("8:30 PM")
        );
    }

    #[test]
    fn schedule_rows_drop_ended_visits_keep_cancelled_and_mark_first_active_as_next() {
        let day: PracticeDayResponse = serde_json::from_value(serde_json::json!({
            "current_time": "2026-08-27T17:00:00.000Z",
            "time_zone": "America/New_York",
            "source_state": { "state": "usable" },
            "visits": [
                {
                    "patient_workspace_id": "pw-ended",
                    "patient_display_name": "Ended Patient",
                    "label": "Morning visit",
                    "starts_at": "2026-08-27T13:00:00.000Z",
                    "ends_at": "2026-08-27T13:30:00.000Z",
                    "status": "scheduled",
                    "preparation": { "state": "prepared", "briefing_id": "b-old" }
                },
                {
                    "patient_workspace_id": "pw-cancelled",
                    "patient_display_name": "Cancelled Patient",
                    "label": "Annual physical",
                    "starts_at": "2026-08-27T17:30:00.000Z",
                    "ends_at": "2026-08-27T18:00:00.000Z",
                    "status": "cancelled",
                    "preparation": { "state": "cancelled", "historical_briefing_id": "b-hist" }
                },
                {
                    "patient_workspace_id": "pw-ready",
                    "patient_display_name": "Ready Patient",
                    "label": "Follow-up",
                    "starts_at": "2026-08-27T18:00:00.000Z",
                    "ends_at": "2026-08-27T18:30:00.000Z",
                    "status": "scheduled",
                    "preparation": {
                        "state": "prepared",
                        "briefing_id": "b-ready",
                        "visit_card_orientation": "A1c rose to 8.4% after stopping metformin."
                    }
                },
                {
                    "patient_workspace_id": null,
                    "patient_display_name": null,
                    "label": "New patient visit",
                    "starts_at": "2026-08-27T19:00:00.000Z",
                    "ends_at": "2026-08-27T19:30:00.000Z",
                    "status": "scheduled",
                    "preparation": { "state": "unavailable" }
                }
            ]
        }))
        .expect("practice day");

        let rows = schedule_rows(&day, None);
        assert_eq!(rows.len(), 3, "ended visit must drop off");
        assert_eq!(rows[0].patient, "Cancelled Patient");
        assert_eq!(rows[0].slot_order, NextVisitSlot::Later);
        assert_eq!(
            rows[0].state,
            ScheduleVisitState::Cancelled {
                historical_briefing_id: Some("b-hist".to_owned())
            }
        );

        assert_eq!(rows[1].patient, "Ready Patient");
        assert_eq!(rows[1].slot_order, NextVisitSlot::Next);
        assert_eq!(
            rows[1].activation(),
            ScheduleVisitActivation::OpenBriefing {
                workspace_id: "pw-ready".to_owned(),
                briefing_id: "b-ready".to_owned(),
            }
        );

        assert_eq!(rows[2].patient, "New patient visit");
        assert_eq!(rows[2].slot_order, NextVisitSlot::Later);
        assert_eq!(rows[2].state, ScheduleVisitState::Unavailable);
    }

    #[test]
    fn refresh_failed_freshness_formats_as_of_in_browser_zone() {
        let day: PracticeDayResponse = serde_json::from_value(serde_json::json!({
            "current_time": "2026-08-27T17:00:00.000Z",
            "time_zone": "America/New_York",
            "source_state": {
                "state": "usable",
                "freshness": "refresh_failed",
                "snapshot_as_of": "2026-08-27T15:00:00.000Z"
            },
            "visits": []
        }))
        .expect("practice day");

        let la = schedule_pane_freshness(Some(&day), ReadStage::Showing, "America/Los_Angeles");
        assert_eq!(
            la,
            SchedulePaneFreshness::RefreshFailed {
                copy: "Coda couldn’t refresh the schedule. Showing visits as of Today, 8:00 AM."
                    .to_owned()
            }
        );
    }

    #[test]
    fn ended_cancelled_and_moved_briefings_show_historical_standing_notices() {
        let ended_item = BriefingSummaryItemWire {
            briefing_id: "b-ended".to_owned(),
            appointment_id: "appt-ended".to_owned(),
            visit_label: "Morning lab review".to_owned(),
            starts_at: "2026-08-27T14:00:00.000Z".to_owned(),
            ends_at: "2026-08-27T14:30:00.000Z".to_owned(),
            prepared_at: "2026-08-27T13:30:00.000Z".to_owned(),
            visit_card_orientation: "Routine morning labs.".to_owned(),
            visit_standing: BriefingVisitStandingWire::Ended {
                ended_at: "2026-08-27T14:30:00.000Z".to_owned(),
            },
        };
        let (badge, detail) =
            history_standing_summary(&ended_item, "2026-08-27T16:00:00.000Z", "America/New_York");
        assert_eq!(badge, Some(("Past visit", Tone::Neutral)));
        assert_eq!(
            detail,
            "Completed visit (Today, 10:00 AM) · Kept in history"
        );
        assert_eq!(
            standing_banner(
                &ended_item.visit_standing,
                "Today, 10:00 AM",
                "2026-08-27T16:00:00.000Z",
                "America/New_York"
            ),
            Some((
                Tone::Neutral,
                "This visit has ended.".to_owned(),
                "Kept in history for the scheduled slot (Today, 10:00 AM).".to_owned(),
            ))
        );
    }
}
