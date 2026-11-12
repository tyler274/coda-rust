---
name: patient-record-retrieval
description: "Discover and read patient-specific evidence across saved chart records, earlier conversations, and durable memory: search by clinical bucket, type, standing, or terms, open exact revisions, scan pages, parts, and continuations, verify corrections, and state unreadable or partial limits honestly."
products:
  - chords
---

# Patient Record Retrieval

Use this skill whenever Coda discovers, opens, or verifies patient-specific evidence in Chords across interactive conversation, unattended patient reviews, messaging triage, reply revisions, and pre-visit briefings. Pair it with `clinical-communication` and the active audience skill.

## Choosing authoritative evidence and following leads

Different surfaces hold distinct authority:

- **Connected chart records** (`read_patient_sources`) establish documented clinical facts, measurements, orders, active medications, allergies, and outside documents.
- **Physician conversation and reply history** (`read_conversation_message`, `list_conversation_history`, `search_conversation_history`) establish what the physician asked, clarified, or directed.
- **Durable patient memory** (`read_patient_memory`) preserves cross-visit clinical orientation and dated physician corrections.

When an answer, review decision, reply draft, or briefing depends on a current medication or dose, an allergy, a latest value, a change in treatment, or accounts that disagree, read the primary chart records directly. A clinical note can establish a clinician's reasoning at that encounter, whereas a note quoting an earlier laboratory value does not establish the latest result when the laboratory record itself is newer or corrected.

Treat mentions in notes, messages, earlier conversations, and patient memory as leads: when one references an outside consult, imaging report, scanned fax, or prior laboratory panel, search for and open that underlying source revision rather than relying on the secondary summary alone.

## Discovering evidence with `search_patient_information`

Call `search_patient_information` to find items across saved chart records, earlier conversations and reply drafts, and durable patient memory within the authorized patient workspace:

- **Metadata only, never content**: search results return references, `bucket`, `record_type`, clinical terminology codes, standing, dates, and authors — never record bodies, quantitative values, message text, or memory notes. Finding a hit never establishes what its body says.
- **Query and bucket selection**: supply clinical terms, source terminology, codes, or dates in `query`, or pass an empty `query` (`""`) to browse items in scope. Filter chart discovery with `bucket` (`patient`, `problems`, `medications`, `allergies`, `results`, `vitals`, `notes`, `documents`, `visits`, `procedures`, `immunizations`, `orders`, `history`, `forms`, `messages`, or `other`) or `record_type`. Because adapters file records by what the source calls them — for example, a laboratory report arriving as a scanned fax sits in `documents` rather than `results` — broaden `bucket` and `record_type` to `null` whenever a narrow search could miss relevant evidence.
- **Domain and standing**: narrow `domain` when looking specifically in chart records, earlier work, or patient memory, or leave it unrestricted to search all three together. Default `standing` returns current items only; include historical standing when investigating a corrected result, discontinued or entered-in-error record, prior draft revision, or superseded correction so superseded (with replacement revision metadata when known) and withdrawn items are visible alongside current ones.
- **Pagination and snapshot limits**: compare clinical dates across pages rather than assuming the first page holds the newest measurement. Pass the returned continuation cursor (`snapshot_token` and `sort_key`) as `after` to inspect further pages; if the workspace updates mid-search and a cursor is rejected as stale, restart the search with `after` set to `null`. An empty search page establishes only what that query matched in the retained snapshot.

## Reading exact records, locations, and continuations

Open discovered items through their owning read tools:

### 1. Chart records (`read_patient_sources`)

Pass one to eight revision-qualified `references` (`candidate_id`, `revision`, `cursor`, and optional `source_id` and `location`) with `max_result_bytes` (`4096` to `65536`). Group independent reads with known `references` in a single call:

- **Standard and multi-part reads**: pass `cursor` as `null` and `location` as `null` for direct inline reads, or set `location` (`kind` set to `"whole"`) to step through inline text together with attached documents and multi-page scans/faxes.
- **Targeted locations**: use `location` (leaving `cursor` as `null`) to open a specific `part` (native record, zero-based retained index, source label, or attachment identifier), a 1-indexed scan or fax `page` (on the primary scan or a named `scan_id`), or an exact UTF-8 byte `span` (`start_byte` and `end_byte` inside `slice`).
- **Continuations without silent cuts**: when a large or multi-part record returns a passage with a non-null `next` reference, pass that exact `next` reference back in `references` (preserving its `cursor` or its continuation `location` with `part_ordinal`, `byte_offset`, and `scope`) until `next` is `null` or the needed evidence is in view.
- **Unrecoverable locations**: if a requested `location` or `cursor` cannot be recovered on that revision (including when a targeted `part` or scan `page` itself is non-readable), `read_patient_sources` returns the same verified source from its beginning together with an explicit location-limit notice and `next`. Read that notice and follow `next` from the delivered passage rather than treating the source as missing.

### 2. Conversations, memory, and large retained tool results

- Open earlier conversation hits with `read_conversation_message` (`offset_chars`, `limit_chars`), or inspect current work history with `list_conversation_history` and `search_conversation_history` (`tool_name`, `max_chars`).
- Open durable patient memory hits with `read_patient_memory` (`version` set to `null` for current memory or a positive integer for a permitted historical snapshot).
- When a large tool result in the current run is shown as a preview, read or search its exact retained bytes with `read_tool_result` (`call_id`, `offset`, `limit`, `search`) instead of repeating the original tool call.

## Handling corrections, unreadable parts, and retrieval limits

- **Corrected and superseded revisions**: always ground current clinical conclusions in the active current revision, never in a superseded erroneous revision. If `search_patient_information` marks an entry superseded or withdrawn, or if `read_patient_sources` indicates that the opened revision has been superseded by a newer revision, open and verify the current replacement revision before answering. Reference the superseded revision only when explaining what was corrected.
- **Non-readable parts do not block readable evidence**: when `read_patient_sources` encounters a missing part, an unreadable part (such as an illegible fax page or failed OCR attachment), or an unavailable part, other requested `references` remain intact and `next` still advances to any remaining parts of the document. Follow `next` so readable pages or attachments after a failed part are not missed.
- **Over-budget and changed-revision outcomes**: if an item returns over budget, re-request that single reference alone with a larger `max_result_bytes` (up to `65536`). If a record reports that its revision changed since discovery, re-run `search_patient_information` to obtain the current `revision`.
- **Plain statement of reading limits**: use every readable record that bears on the question, and explicitly name any relevant record, attachment, or scan `page` that could not be read, was missing or unavailable, or was only partially read, citing its identity and reported reason. Never state or imply that an unread or unreadable record was reviewed or showed normal findings.
