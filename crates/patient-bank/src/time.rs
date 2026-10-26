//! The practice's clock.
//!
//! Ledgers record Eastern wall-clock times. Arithmetic and comparisons happen on
//! the wall clock; only FHIR output converts to UTC. A wall time in the
//! spring-forward gap or the fall-back fold resolves with the offset before the
//! transition (jiff's "compatible").

use jiff::civil::Date;
use jiff::tz::TimeZone;
use jiff::{Span, Timestamp};

use crate::ledger::Stamp;

/// The practice's "today": every record is before it, and only booked visits are
/// after it. What happens next is decided later, not authored.
pub const AS_OF: &str = "2026-08-27T12:00";

pub fn as_of() -> Stamp {
    Stamp::parse(AS_OF).expect("AS_OF is a stamp")
}

/// The release time of a result that is not back yet.
pub fn never() -> Stamp {
    Stamp(jiff::civil::DateTime::MAX)
}

fn eastern() -> TimeZone {
    TimeZone::get("America/New_York").expect("bundled tz database has America/New_York")
}

/// The FHIR instant (`YYYY-MM-DDTHH:MM:SS+00:00`) of an Eastern wall-clock time.
pub fn utc(at: Stamp) -> String {
    let zoned =
        at.0.to_zoned(eastern())
            .expect("representable Eastern time");
    zoned
        .timestamp()
        .strftime("%Y-%m-%dT%H:%M:%S+00:00")
        .to_string()
}

/// A FHIR UTC instant on the Eastern wall clock, to the minute.
pub fn eastern_of(utc: &str) -> Option<Stamp> {
    let ts: Timestamp = utc.parse().ok()?;
    let dt = ts.to_zoned(eastern()).datetime();
    Some(Stamp(
        dt.with().second(0).subsec_nanosecond(0).build().ok()?,
    ))
}

pub fn plus(at: Stamp, span: Span) -> Stamp {
    Stamp(at.0.checked_add(span).expect("date in range"))
}

/// Whole years between two dates (an age).
pub fn age_on(birth: Date, on: Date) -> i64 {
    let mut years = i64::from(on.year() - birth.year());
    if (on.month(), on.day()) < (birth.month(), birth.day()) {
        years -= 1;
    }
    years
}

/// Wall-clock minutes from `a` to `b`.
pub fn minutes_between(a: Stamp, b: Stamp) -> i64 {
    a.0.duration_until(b.0).as_secs() / 60
}
