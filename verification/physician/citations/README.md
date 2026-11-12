# Citations

How Coda cites the sources behind its work, how you, a physician, read a citation and open its source, and how Basis and limitations sits beside it. The same rules hold wherever you read Coda's work: an answer, a review, a reply draft, a briefing. [physician.md](../../../intent/physician.md#p1-ask-about-a-patient-with-evidence) links here.

## Words

- **Work:** one piece of Coda's writing that you read: an answer, a review, a reply draft or a briefing.
- **Source:** one thing Coda read: a record, an article, a wearable reading, a patient message.
- **Version:** the exact revision of a source that Coda read.
- **Label:** the short name Coda's harness gives one version of a source when the model reads it. The model cites by writing the label beside a claim.
- **Delivered:** a source, or a passage of it, is delivered while its text is in what the model is working from. After Coda starts from a fresh context, an earlier read is delivered again only when Coda reads it again.
- **Citation:** a numbered mark beside a claim. It points to one version of one source and to the place in it that the claim rests on.
- **Place:** the part of a source a citation points to: a passage, a field, or a page.
- **Passage:** words quoted from the source's own text, with where they are in it.
- **Field:** one value in a structured record, identified by its path in the record.
- **Page:** one page of a scan or fax that has no text to quote.
- **Preview:** Coda's short description of a cited source, written when Coda finishes the work. Its date is when Coda read that version for the work.
- **Finish:** the one moment Coda hands in the work, with its citations and its Basis and limitations.
- **Panel:** the side panel that shows a source as a readable record, beside the work.
- **Layout:** a saved arrangement for one source and one record type. It places the record's fields and never holds a value.
- **Basis and limitations:** Coda's account of what the work rests on and what is missing, uncertain or assumed.
- **Sources:** the control that lists each cited source once.

## States and steps

Coda, writing the work:

| From | Step | To |
| --- | --- | --- |
| Working | Coda reads a source | That version has a label and is delivered, and Coda can cite it |
| Working | Coda finishes: every label names a delivered source, every citation names its place, every cited source has a preview, every passage is in the delivered text, every field is in the delivered record, and the Basis is written | Finished: the work is saved with its citations and its Basis |
| Working | Coda finishes with a label it was never given or whose source is no longer delivered, a citation that names no place, a cited source without a preview, a passage that isn't in the delivered text, a field the record doesn't have, or no Basis | Refused: Coda is told what is wrong and corrects it in the same run |
| Working | the work is stopped or cut off | Not finished: no citations and no Basis are saved |

You, reading a citation:

| From | Step | To |
| --- | --- | --- |
| The mark shows | you point at it | The preview shows, marked as prepared by Coda with its date, with the passage or field quoted, or the page named |
| The mark shows | you choose it | The panel is loading |
| Loading | the source loads, and the place is found | The panel shows the version Coda read, at the place, highlighted |
| Loading | the source loads, and the place can't be found in the readable record | The panel shows the same version without highlighting, with a short notice |
| Loading | the source can't load | The panel says so, with Retry |
| Loading | you may no longer access the patient, or the content was withheld after a wrong-patient correction | The panel says the source is unavailable, with the reason |
| The panel shows a source | you choose another citation | Loading that citation |
| The panel shows a source | you may no longer access the patient | The panel says the source is unavailable, with the reason |
| The panel shows a source | you close it | The mark shows |

How the panel reads a record:

| From | Step | To |
| --- | --- | --- |
| A record to show | the source supplies its own readable text | The panel shows that text |
| A record to show | there is no such text, and a saved layout fits the source and record type | The panel shows the record arranged by the layout |
| A record to show | there is neither | The panel shows the general view: every field under its name |
| A record type with no layout | Coda makes a layout, and it passes its checks | The layout is saved for that source and record type |
| A record type with no layout | the layout fails a check, or it can't be made | No layout is saved, and the general view is used |

## Rules

1. Every citation in finished work points to a version of a source and to a place in it, and both were delivered when Coda finished.
2. The label is the only part of a citation that is read from the model's text: previews, passages, fields and links never come from it.
3. Every cited source in finished work has a preview.
4. In one piece of work, one version of a source has one number however many claims cite it, and the Basis and limitations uses the same numbers.
5. Each citation opens its own place, also when several citations share a source.
6. A citation opens the version Coda read, also after the source has changed.
7. The panel highlights only the place the citation names, and never picks one because its text is equal.
8. Every value the panel shows is copied from the record, and no value comes from the model or from a layout.
9. Nothing in the record is dropped: a field no layout places still shows, under its source's name for it.
10. A part of a record reads as not supplied only when the source says it is absent.
11. A source reads as unavailable only when it can't be read or shown, never because the model said so.
12. The panel shows a source's content only while you may access its patient.
13. A late answer for an earlier citation never replaces the source the panel is showing for a later one.
14. Finished work has exactly one Basis and limitations, and work that was stopped or cut off has no citations and no Basis.
15. Basis and limitations is never part of a message to a patient.

These rules rest on the code, its records and the screen, not on the order of steps, so the model doesn't check them:

- Coda's words and the source's words never look alike. The preview is marked as prepared by Coda, with its date, and the record reads as the record.
- The panel shows the record's own status beside its kind and date, and an inactive, refuted, entered-in-error or preliminary record says so before its statement.
- A structured record's raw data stays under Technical details and shows nowhere else.
- The live link into the EHR comes from the source's adapter and shows only when the adapter supplies one.
- Sources lists each cited source once.
- Basis and limitations sits collapsed beside Sources and opens in place. It says what the work rests on and what could change it, and it is never a generic disclaimer.
- A citation and its source reopen later as they were. The panel shows the source as of the work, with that date, and doesn't mark that a newer version exists.

## Outside this spec

What reaches the model, and how the harness gives labels, follow [harness.md](../../../intent/harness.md#guarantees). How records are read from each source and filed follows [physician.md](../../../intent/physician.md). How an answer ends, and what a stopped or partial answer shows, follow [chat](../chat/README.md).
