//! What happens after the bank time: dated arrivals and the practice calendar.
//!
//! Ledgers run to `time::HORIZON`. The bank chart is the chart as of `time::AS_OF`;
//! everything that reaches the practice later is an arrival, a new version of a
//! resource with `meta.lastUpdated` at the moment it became available (a result
//! released, a letter received, a note signed). Both come from the same renderer,
//! so an arrival is exactly how the chart changes at that moment.
//!
//! The calendar is the practice's own visits from the bank day to the horizon as
//! FHIR Appointments: fulfilled, booked, cancelled or no-show.

use std::collections::HashMap;

use jiff::ToSpan;
use serde_json::Value;

use crate::fhir::{Appointment, CodeableConcept, Meta, Participant, Reference};
use crate::ledger::{Missed, Patient};
use crate::render::{Renderer, opaque};
use crate::seeds::Seed;
use crate::time;
use crate::timeline::Timeline;

/// A resource without its `meta`, for comparing versions.
fn body(line: &str) -> (String, Value) {
    let mut v: Value = serde_json::from_str(line).expect("rendered FHIR is JSON");
    v.as_object_mut().expect("a resource").remove("meta");
    let id = v["id"].as_str().unwrap_or("").to_string();
    (id, v)
}

/// Every resource version that reaches the practice after the bank time, in order,
/// as (family, resource).
pub fn arrivals(r: &Renderer, seed: &Seed, p: &Patient) -> Vec<(&'static str, Value)> {
    let (as_of, horizon) = (time::as_of(), time::horizon());
    let (now, _) = r.render(seed, p, as_of);
    let mut seen: HashMap<String, (Value, u32)> = HashMap::new();
    for (_, lines) in now.lines() {
        for line in lines {
            let (id, v) = body(&line);
            seen.insert(id, (v, 1));
        }
    }
    let mut out = Vec::new();
    for at in Timeline::new(p)
        .instants()
        .into_iter()
        .filter(|s| *s >= as_of && *s < horizon)
    {
        let (chart, _) = r.render(seed, p, time::plus(at, 1.minutes()));
        for (fam, lines) in chart.lines() {
            for line in lines {
                let (id, mut v) = body(&line);
                let version = match seen.get(&id) {
                    Some((old, _)) if *old == v => continue,
                    Some((_, n)) => n + 1,
                    None => 1,
                };
                seen.insert(id, (v.clone(), version));
                v.as_object_mut().expect("a resource").insert(
                    "meta".into(),
                    serde_json::json!({
                        "versionId": version.to_string(),
                        "lastUpdated": time::utc(at),
                    }),
                );
                out.push((fam, v));
            }
        }
    }
    out
}

/// The patient's visits on the practice calendar from the bank day to the horizon.
pub fn calendar(seed: &Seed, p: &Patient) -> Vec<Appointment> {
    let (as_of, horizon) = (time::as_of(), time::horizon());
    let mut out = Vec::new();
    for ev in Timeline::new(p).events {
        let Some(minutes) = ev.kind.booked_minutes() else {
            continue;
        };
        if !ev.booked() || ev.start.date() < as_of.date() || ev.start >= horizon {
            continue;
        }
        let end = ev
            .end
            .unwrap_or_else(|| time::plus(ev.start, minutes.minutes()));
        let label = ev.kind.encounter().map_or("Visit", |(_, _, l)| l);
        out.push(Appointment {
            resource_type: "Appointment",
            id: opaque("appt", &[&seed.mrn, &ev.id]),
            meta: Meta::on(&time::utc(as_of)),
            status: match ev.appointment {
                Some(Missed::Cancelled) => "cancelled",
                Some(Missed::NoShow) => "noshow",
                None if ev.start >= as_of => "booked",
                None => "fulfilled",
            },
            service_type: vec![CodeableConcept::text(ev.service())],
            appointment_type: CodeableConcept::text(label),
            description: ev.reason.clone().unwrap_or_else(|| label.to_string()),
            start: time::utc(ev.start),
            end: time::utc(end),
            minutes_duration: time::minutes_between(ev.start, end),
            participant: vec![Participant {
                actor: Reference::to("Patient", &seed.mrn),
                status: "accepted",
            }],
        });
    }
    out
}
