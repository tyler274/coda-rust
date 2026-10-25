//! The practice calendar: its own visits from the bank day on as FHIR Appointments.
//!
//! The bank chart is the chart as of `time::AS_OF`, and nothing after it is
//! authored except visits already booked. Today's visits are fulfilled (or
//! cancelled or missed); later ones are booked, with nothing in them yet.

use jiff::ToSpan;

use crate::fhir::{Appointment, CodeableConcept, Meta, Participant, Reference};
use crate::ledger::{self, Patient};
use crate::render::opaque;
use crate::seeds::Seed;
use crate::time;
use crate::timeline::Timeline;

/// The patient's visits on the practice calendar from the bank day on.
pub fn visits(seed: &Seed, p: &Patient) -> Vec<Appointment> {
    let as_of = time::as_of();
    let mut out = Vec::new();
    for ev in Timeline::new(p).events {
        let Some(minutes) = ev.kind.booked_minutes() else {
            continue;
        };
        if !ev.booked() || ev.start.date() < as_of.date() {
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
                Some(ledger::Appointment::Booked) => "booked",
                Some(ledger::Appointment::Cancelled) => "cancelled",
                Some(ledger::Appointment::NoShow) => "noshow",
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
