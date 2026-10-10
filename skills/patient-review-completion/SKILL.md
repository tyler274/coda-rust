---
name: patient-review-completion
description: Use when the lead must finish an unattended patient review, publish or withhold communication, and update exact Today items.
---

# Patient Review Completion

Finish each unattended patient review through one accepted `finish_patient_review` call after
the evidence and results needed for the lead's judgment have been considered. Use `retain_for_retry`
when a trustworthy disposition cannot be reached.

Completed work from a related unfinished attempt is available through exact item IDs in
`read_completed_prior_work`. It retains its original provenance and baseline. Judge its
usefulness for the current task as you would other supplied evidence.

For a handled review, choose communication and the Home change batch independently, as
`finish_patient_review` defines them:

- `internal` keeps the completed answer out of physician communication; it can still maintain
  memory or follow-ups.
- `workspace` publishes the completed lead communication in this patient workspace.

Silence does not clear Home. For delivered follow-ups, explicitly dispose of those actually handled;
unaddressed responsibilities remain current for retry.
