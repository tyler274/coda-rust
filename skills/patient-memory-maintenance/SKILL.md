---
name: patient-memory-maintenance
description: Use when assessing, creating, or updating durable patient understanding, its paired physician synopsis, or dated working context, including preventive care and unresolved findings.
---

# Patient Memory Maintenance

Use this skill for initialization, ordinary clinical work, and reconciliation. The operating
index is the quick source of truth for salient care plans, their reasons, physician direction and
the follow-ups Coda is tracking. It is never the ground truth for medications, latest values or
anything else the chart owns; those are always re-read from the chart. The paired synopsis gives
the physician a concise orientation. The references guide organization without imposing a
clinical schema.

## Coverage and scope

Initialize broadly and challenge omissions during broader reviews. Focused questions and source
changes call for relevant investigation, not a repeated full-chart review. Quiet reconciliation
retains its assigned memory-maintenance scope.

Keep preventive and monitoring care represented, including uncertainty about completion. Preserve
relevant incidental findings and documented follow-through even when absent from the problem list
or outside the physician's specialty. Judge significance and investigation depth from the task
and evidence.

An index entry creates no concern, scheduled follow-up, or external action. Use the corresponding
capability when warranted; keep follow-up rationale here and its schedule and lifecycle there.

## Maintaining the pair

Write the index and its `physician_synopsis` with `replace_patient_operating_index`, whose definition
says how a write and its edits work. Copy each `old_text` exactly from the delivered index, long enough
to match once, so every removal is an explicit edit. Update consequential
understanding during ordinary work rather than waiting for reconciliation; leave unchanged memory
alone. Retain supported active problems, treatment and the reasons for its changes, allergies and
reactions, important history, and unresolved follow-through. Medications and known doses may stay
as orientation, each dated and cited to its source ("per the 2026-02 medication list"). Any answer,
reply, preparation or action that depends on a current medication or latest value re-reads the
chart first; never quote the index as the current list.

Coda's own earlier index, logs and messages never establish a clinical fact (a medication, value, result or diagnosis): cite the chart, or the physician's own words for their direction.

A patient's memory reflects only their own evidence; a message withdrawn from their record by a link correction is no longer theirs, so nothing it said belongs in their memory. An index or log that relied on such a message reaches you only as its place: rewrite a withheld current index from this patient's own record.

Use `write_patient_daily_log` when useful dated context changes:
physician direction, corrections, decisions, or observed follow-through. Logs retain relevant history
but do not correct a superseded index or synopsis. The runtime owns reconciliation coverage outside
clinical Markdown; initialization and reconciliation retain their assigned scope.

The memory write tools say when a citation already in the version you edit may stay. Read the exact
source in this run before adding a citation, and never attach a carried citation to a new or changed
claim. Apply the resident evidence
contract throughout; identify physician-provided information separately from connected records.

## Physician synopsis

Start with the patient one-liner, then short problem sections when needed; a brief synopsis can
remain a paragraph. Select essential supported history, current state, and material unresolved
questions. Include preventive care when it changes understanding or the next decision. Preserve
whole-patient orientation rather than copying the index, a visit preparation, or the latest reply.
Distinguish last-documented treatment from confirmed use, and recommendations from accepted or
completed care. Do not invent unresolved work. Follow physician-clinical-communication for Markdown,
claim-level citations, and physician-readable basis.

## Supporting references

Select applicable resources through `skill_view`:

- [Authoring guide](references/authoring-guide.md): one-liner, problem inventory, timelines,
  current status, and outstanding issues.
- [Preventive care](references/preventive-care.md): eligibility, intervals, missingness,
  monitoring, and incidental-finding follow-through.
- [Specialty review](references/specialty-review.md): cardiovascular, oncology, and GI detail.
- [Clinical abbreviations](references/clinical-abbreviations.md): readable, unambiguous terminology.
