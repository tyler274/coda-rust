//! The practice's clock.
//!
//! Ledgers record Eastern wall-clock times (`YYYY-MM-DDTHH:MM`). Arithmetic and
//! comparisons happen on the wall clock, as the Python pipeline did with aware
//! datetimes sharing one tzinfo; only the edge converts to the UTC strings the
//! FHIR resources carry. A wall time in the spring-forward gap or the fall-back
//! fold resolves with the offset before the transition (Python's `fold=0`,
//! jiff's "compatible").

use jiff::civil::{Date, DateTime};
use jiff::tz::TimeZone;
use jiff::{Span, Timestamp, ToSpan};

pub type Local = DateTime;

fn eastern() -> TimeZone {
    TimeZone::get("America/New_York").expect("bundled tz database has America/New_York")
}

/// Parse a ledger stamp: `YYYY-MM-DDTHH:MM` (anything after minutes is
/// ignored, as Python's `local[:16]` did) or a bare `YYYY-MM-DD` (midnight).
pub fn local(stamp: &str) -> Option<Local> {
    let head = stamp.get(..16).unwrap_or(stamp);
    if head.len() == 10 {
        return head.parse::<Date>().ok().map(|d| d.at(0, 0, 0, 0));
    }
    head.parse::<DateTime>().ok()
}

/// `YYYY-MM-DDTHH:MM`
pub fn fmt_local(dt: Local) -> String {
    dt.strftime("%Y-%m-%dT%H:%M").to_string()
}

/// Eastern wall clock to the `YYYY-MM-DDTHH:MM:SS+00:00` FHIR instant.
pub fn to_utc(dt: Local) -> String {
    let zoned = dt.to_zoned(eastern()).expect("representable Eastern time");
    zoned
        .timestamp()
        .strftime("%Y-%m-%dT%H:%M:%S+00:00")
        .to_string()
}

/// `to_utc` of a ledger stamp.
pub fn stamp_to_utc(stamp: &str) -> Option<String> {
    local(stamp).map(to_utc)
}

/// A FHIR UTC instant (`...+00:00`) on the Eastern wall clock.
pub fn utc_to_local(utc: &str) -> Option<Local> {
    let ts: Timestamp = utc.parse().ok()?;
    Some(ts.to_zoned(eastern()).datetime())
}

pub fn minutes(n: i64) -> Span {
    n.minutes()
}

pub fn hours(n: i64) -> Span {
    n.hours()
}

pub fn days(n: i64) -> Span {
    n.days()
}

/// Wall-clock addition, as Python's `aware_datetime + timedelta`.
pub fn add(dt: Local, span: Span) -> Local {
    dt.checked_add(span).expect("date in range")
}

/// Whole years between two dates (an age).
pub fn age_on(birth: Date, on: Date) -> i64 {
    let mut years = i64::from(on.year() - birth.year());
    if (on.month(), on.day()) < (birth.month(), birth.day()) {
        years -= 1;
    }
    years
}

/// `fromisoformat(a) - fromisoformat(b)` for UTC instants, in seconds.
pub fn utc_seconds_between(later: &str, earlier: &str) -> Option<i64> {
    let a: Timestamp = later.parse().ok()?;
    let b: Timestamp = earlier.parse().ok()?;
    Some(a.as_second() - b.as_second())
}
