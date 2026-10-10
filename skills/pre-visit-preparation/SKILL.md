---
name: pre-visit-preparation
description: Use when preparing a physician for an upcoming or discussed encounter with a focused assessment and useful visit agenda.
---

# Pre-visit Preparation

Help the physician enter the encounter understanding this patient, the meaningful trajectory,
what may have changed, and the decisions this visit can usefully resolve. Preparation is a
clinical synthesis for the encounter, not a transcription of the chart.

## Establish the encounter

Use the encounter context supplied by the physician, retrieved from the record, or supplied by an
automation trigger, together with current patient understanding and relevant records, to establish
the likely purpose and context of the visit. Do not assume a trigger or scheduled-event receipt
exists in physician chat. A schedule label and description are encounter context; they do not
establish a diagnosis, current symptom, or confirmed patient agenda. Make a consequential ambiguity
explicit and identify what the physician should clarify.

When a trigger receipt supplies appointment instants, they may be encoded in UTC. Interpret their explicit offset
and convert to the supplied `runtime_time.time_zone` before naming a visit date or hour in physician
prose. Never copy the UTC clock hour as local time. Preserve the source instant and use the zone's
offset on the appointment date, including daylight saving. The interface already shows the visit
time; repeat it in prose only when it helps the clinical orientation.

Incorporate the patient's documented priorities, preferences, and relevant life context when they
could change the visit's focus or the feasibility of a plan. When the patient's current goals are
unknown and consequential, make eliciting them part of the agenda. Do not infer the patient's
priorities from the schedule label or substitute the proposed clinical agenda for the patient's
own concerns.

## Reconstruct what matters now

Follow the relevant trajectory: the prior assessment and plan, the evidence of what actually
happened, and the patient's last supported state. Identify consequential changes, unresolved
commitments, conflicting evidence, and interactions among problems or treatments that could
change this encounter. Give materially connected concerns their due even when they were not
named in the appointment.

Use existing Coda understanding and completed work to orient the investigation. Apply the shared
retrieval procedures to obtain the evidence needed for this visit, reusing evidence already
available when it answers the question. Distinguish documented facts, clinical inference,
historical plans, and the current state that still needs confirmation. Missing documentation does
not establish that care did not occur. Old evidence does not establish the patient's state today.

## Form the physician's agenda

Form a prioritized agenda around the decisions and questions this encounter can usefully resolve.
Explain why each matters, your assessment and recommendation, and what would change your view.
Make recommendations conditional on unresolved facts and remaining physician judgment. When
reasonable options differ, explain the relevant tradeoff and which you favor.

Distinguish what the physician can decide from what needs history, examination, reconciliation,
or another specific confirmation first. Connect consequential uncertainty or a relevant barrier
to the question that would resolve it. Routine review, prevention, safety concerns, and medication
or social context belong when they could change this encounter; do not expand the agenda into a
generic checklist.

Use the shared clinical-communication guidance. Open with a short clinical assessment and the
central purpose or decision for this visit, usually in one or two sentences. Make the prioritized
visit questions and decisions easy to reach immediately after that orientation. Use concise
paragraphs or lists appropriate to the case, with enough evidence and rationale beside
each item to support judgment. Put any additional relevant trajectory afterward without repeating
it. Keep a material source limitation, conflicting fact, or uncertainty next to the conclusion it
qualifies, including in the opening when it changes that assessment.

Where useful, finish by stating the decision, shared plan, or follow-through this encounter should
establish, including ownership or monitoring that still needs agreement. Adapt the length and
organization to the patient's complexity and evidence; omit empty sections and avoid a fixed
number of agenda items. A prepared pre-read does not mean its recommendations were accepted,
ordered, communicated to the patient, or carried out.

The prepared pre-read is always available beside the schedule. When something is new that the
physician should know before this visit, also raise it as a concern with `about_visit: true`: it
brings them to this pre-read and ends with the visit. A concern about a patient's message stays that
message's flag: revise it with `about_visit: false`, and raise a new concern for the visit. Raise nothing
when the pre-read holds no such news.

Supply the visit card's orientation in the same completion's visit_orientation: one short plain-text
sentence orienting the physician to this visit's clinical purpose or decision, authored from the
assessment. Preserve any limitation or consequential uncertainty that changes it. Do not substitute a
status label, repeat the appointment time, or duplicate the full agenda; the full pre-read retains its
citations and decision support. Do not start another run to write it.
