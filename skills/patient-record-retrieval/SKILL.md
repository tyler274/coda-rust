---
name: patient-record-retrieval
description: Use to find patient-specific evidence in the retained chart and read the exact records behind an answer.
---

# Patient Record Retrieval

Identify which evidence can answer the question. Connected records establish documented patient
facts; exact physician statements establish what the physician said or directed. These sources
retain their distinct authority even when they concern the same event.

When the answer depends on a current value, a change in treatment, or accounts that disagree, check
the source records themselves. A note can establish a physician's plan; a note quoting a lab does not
establish the latest result when the lab record itself may be newer or corrected.

Use `search_patient_information` to browse the retained chart or search by a term, code,
or date. An empty query browses the records in scope. A record-type filter can help
distinguish a measured result from a note quoting it. Search across types when a narrow
search misses relevant evidence. The search uses the words you supplied, including matches
in decoded attachment text. Try the source's terminology or code when needed.

Search pages are ordered by candidate ID. To find the latest result, compare the measurement
dates across relevant pages. Continue the same query with its returned cursor until you
have enough evidence. An empty page establishes only what this query found in the retained chart.

Pass the returned reference to `read_patient_sources`. Check what the dated record actually
says before using it in an answer. If the record has changed, search again to get its current
revision. A large read delivers an exact passage and a reference for the next passage. Follow
that reference to reach the part you need. Identify the parent record when citing decoded text.
An encoded attachment alone cannot establish what it says.

Name the source and date beside the claim it supports. Say when a record was missing, could
not be read, or was only partly read if that limits the answer.
