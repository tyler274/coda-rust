---
name: patient-record-retrieval
description: Use when patient-specific facts or context require source discovery, acquisition, exact reads, or memory/history lookup.
---

# Patient Record Retrieval

Identify which evidence can answer the question. Connected records establish documented patient
facts; exact physician statements establish what the physician said or directed; Coda memory
orients the investigation and preserves prior understanding. Medical-knowledge questions use the
separate clinical-evidence capability when needed and available. These sources retain their
distinct authority even when they concern the same event.

When the answer depends on a current value, a change in treatment, or accounts that disagree, check
the source records themselves. A note can establish a physician's plan; a note quoting a lab does not
establish the latest result when the lab record itself may be newer or corrected.

For an initial assessment, broader patient review, or relevant problem investigation, the
`patient-memory-maintenance` skill provides authoring, preventive-care, and specialty-review
references. Use the applicable guidance to notice gaps and choose what to investigate; the
current index and chart problem list are not an exhaustive account of the patient. These
references do not prescribe a fixed search plan or expand the current assignment's authority.

Family searches discover records within the authorized patient snapshot. Filters select matching
source metadata; results describe that query, not the patient's entire record. Choose filters
and candidates deliberately, and continue a search with its exact returned cursor when needed.
Use the request's recorded source coverage and limits when interpreting results. An available
family may cover only a declared subset; a family not covered is not evidence of absence.

When needed and available, use `acquire_patient_sources` for an additional source query, choosing
from the acquisition families in the current coverage receipt; its definition says what its outcome
establishes and what the next response observes.

Read selected records with `read_patient_sources({ references: [...] })`. References accept
candidate IDs or patient-source citation labels such as `[^n]`. Exact reads return source bytes and
availability for that accepted version at this investigation's source observation. Source-declared references can lead to related
records.

Exact reads keep the native record intact. `content_parts` reports separately decoded text and
unreadable attachments. Cite decoded text through its parent record's exact handle. Retained
PDF/image bytes and an attachment reference alone do not establish readable content.

Use `search_patient_information` for lexical, semantic, or hybrid discovery within its permitted
families. Read source references with `read_patient_sources` and retained operating-index or
daily-log versions with `read_patient_memory_versions`. The lead can also find accepted physician
conversation and completed Coda History through the `history:*` families and read the returned
`entry_ref`s with `read_patient_history`; a worker receives required physician statements from the
lead in its assignment. Semantic indexing can lag; ranked results do
not prove absence or clinical sufficiency. Exact historical versions retain their original
meaning. `read_patient_daily_logs` with an empty date list browses current log dates without
reading their bodies; selected dates return complete current logs or explicit unavailability.
Reconciliation supplies the exact dates required for its review. Logs remain working context.

Distinguish no matching records from records not read, unavailable versions, and failed access.
Use recorded coverage and availability to describe the exact remaining source limitation.
