---
name: patient-memory-maintenance
description: "Maintain durable patient memory across physician conversation, patient reviews, messaging triage, and visit preparation: read memory, re-verify medications and values in the chart, record dated physician corrections distinct from reported facts, patch Coda's private operating index, and keep the patient-page summary current."
products:
  - chords
---

# Patient Memory Maintenance

Use this skill whenever Coda reads or updates durable patient memory during physician conversation, unattended chart or message reviews, or pre-visit preparation. Pair it with `clinical-communication` and the active audience skill for the workflow.

## Scope and boundaries

This skill governs ongoing reading, verification, correction, and incremental upkeep of patient memory. Initial patient understanding when a patient's chart is first connected is a separate workflow; during ordinary conversation, review, messaging triage, and visit preparation, update only the parts of memory that the current evidence or physician input warrants.

Conversation working notes belong only to a single conversation's context window handoff and are separate from durable patient memory. Never rely on conversation working notes to preserve durable clinical understanding or physician corrections across workflows, and never copy temporary turn-by-turn scratchpads into patient memory.

## Reading memory and checking live chart truth

Call `read_patient_memory` (with `version` set to `null` for current memory, or to a positive integer from the permitted version headers when inspecting earlier history) when durable clinical context, prior rationale, or recorded physician corrections bear on the task:

- **Available**: orient from the returned operating index, active physician corrections, and saved summary, then verify live clinical facts in the chart as needed.
- **Absent**: work directly from live chart sources (`search_patient_information` and `read_patient_sources`) and save only the focused understanding warranted by the current work.
- **Withheld**: when a version was retracted (for example, after a message linked to the wrong patient was removed), never reconstruct or carry forward retracted content. Ground clean updates strictly in this patient's own chart records and permitted history.

**Chart truth for medications, allergies, and current values**: Patient memory is never the source of truth for current medications, doses, allergies, or recent laboratory and vital values. Before answering a question, drafting or revising a patient reply, triaging a review item, or preparing a visit briefing that depends on current medications or values, search and re-read the live chart with `search_patient_information` and `read_patient_sources`. If the chart has changed since an earlier memory note was written, follow the live chart and update the stale memory topic so durable orientation matches the record.

## Private operating index versus patient-page summary

Patient memory has two distinct parts with different audiences and tools:

### 1. Private operating index (`save_patient_memory`)

The operating index is Coda's private clinical understanding of the patient and is never shown to the physician. Organize topics flexibly around what matters clinically for this patient rather than filling a rigid template:

- Active problems, treatment rationale, and why prior therapies were adjusted or stopped.
- Preventive care, chronic disease monitoring intervals, and incidental findings or pending follow-through that should not be lost between visits.
- Unresolved clinical uncertainties, discordant outside records, and relevant patient context.

Use `save_patient_memory` with disjoint `upsert` (`topic` and `note`) and `remove` lists to patch only the topics that changed. Topics omitted from `upsert` and `remove` stay intact, and physician corrections and the patient-page summary remain untouched. Attribute clinical facts inside notes to their dated sources so future runs know what to re-verify.

### 2. Physician-facing patient summary (`save_patient_summary`)

The patient summary is shown on the patient page. Never place private operating-index shorthand or internal notes into `save_patient_summary`.

Rewrite the summary with `save_patient_summary` when a clinical change or physician correction materially alters what the physician needs to see on the patient page, or during periodic review when any cited source has changed:

- Write concise, clinically grounded prose in `content`.
- Attach exact `references`: chart record references (`source_id`, `candidate_id`, `revision`, `label`, and `record_date`), conversation message references (`chat_id`, `message_id`, `label`, and `stated_on`), or literature references (`citation` and `identifier`).
- Populate `dates` (`written_at`, `reviewed_at`, and `pulled_at`) using `YYYY-MM-DD` calendar dates (or `null` when `reviewed_at` or `pulled_at` does not apply). Those dates state only when the summary was written, reviewed, and pulled; none of them claims the patient is clinically stable.

## Recording physician corrections and direction (`record_patient_correction`)

When the physician corrects Coda, clarifies patient history, or states clinical direction — in chat, on a reply, or on a concern — handle it deliberately:

1. **Check verifiable chart questions first**: if the correction concerns a factual question that the chart can answer (such as a recorded result, medication order, or allergy entry), check `read_patient_sources` first. When the chart conflicts with what the physician stated, explain the discrepancy clearly with the dated record rather than silently accepting or dismissing either source.
2. **Keep physician statements distinct from chart-reported facts**: record the physician's correction or direction with `record_patient_correction` so durable memory preserves *what the physician said and when* alongside exact message authorship, separate from EHR-reported facts:
   - Set `recorded_on` as `YYYY-MM-DD` and state what the physician directed or clarified in `statement`.
   - In interactive physician chat, pass `null` for `source_message_id`, `source_chat_id`, and `source_physician_id` so the runtime binds the current authored message automatically. In background jobs acting on a saved physician message, pass the explicit `source_message_id`, `source_chat_id`, and `source_physician_id`.
   - When the physician updates or reverses an earlier recorded correction, set `supersedes_correction_id` to the prior active `correction_id` so history preserves the supersession cleanly.
   - Use `upsert_topics` and `remove_topics` in the same `record_patient_correction` call to update only the affected operating-index topics while keeping all unrelated understanding intact.

## Distinguishing no change from a failed update

Keep quiet non-updates strictly separate from failed writes:

- **No change warranted**: when `read_patient_memory` and live chart reads confirm that durable understanding and summary citations already match current evidence, call **no** memory mutation tool (`save_patient_memory`, `record_patient_correction`, or `save_patient_summary`). Routine lookups and quiet reviews should leave the memory version unchanged rather than rewriting unchanged text.
- **Failed or refused update**: when a memory update *is* warranted and a call to `save_patient_memory`, `record_patient_correction`, or `save_patient_summary` is refused (`executed` is `false`) or fails, durable memory was **not** updated and its version did not advance. Never treat a refused write as "no change needed," and never tell the physician that a correction or memory update was saved when the tool did not execute:
  - If the refusal points to a correctable call issue (such as an invalid date string, overlapping `upsert` and `remove` topic keys, or a missing source message identifier), fix the arguments and verify the returned receipt before finishing.
  - If authority, session state, or storage prevents completing the write, report plainly that durable patient memory could not be updated while still delivering any chart-backed clinical answer you can support.
