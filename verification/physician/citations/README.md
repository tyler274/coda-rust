# Citations

How Coda cites the sources behind its work, how a citation opens its source, and how Basis and limitations sits beside it. The same rules hold wherever you, a physician, read Coda's work: an answer, a review, a reply draft, a briefing. [physician.md](../../../intent/physician.md#p1-ask-about-a-patient-with-evidence) links here.

## Words

- **Work:** one piece of Coda's writing that you read: an answer, a review, a reply draft or a briefing.
- **Source:** one thing Coda read: a record, an article, a wearable reading, a patient message.
- **Version:** the exact revision of a source that Coda read.
- **Label:** the short name Coda's harness gives one version of a source when the model reads it. The model cites by writing the label beside a claim. A label names its version for the whole piece of work.
- **Delivered:** a version, a passage of its text or a page is delivered while it is in what the model is working from. After Coda starts from a fresh context, an earlier read is delivered again only when Coda reads it again.
- **Citation:** a numbered mark beside a claim. It points to one version of one source. When Coda quoted the source, it also points to the place in it that the claim rests on.
- **Place:** the part of a source a citation can point to: one or more passages, or a page. A citation without a place points to its source as a whole.
- **Passage:** words quoted from the source's own text. Differences in spacing, line breaks, quotation marks and dashes do not count. When those words occur more than once in the delivered text, the place is each occurrence.
- **Field:** one value in a structured record. A passage quoted from a structured record is shown as the fields its words cover.
- **Page:** one page of a scan or fax that has no text to quote, named by its number.
- **Valid:** a citation is valid when its label names a version the harness gave Coda in this piece of work.
- **Finish:** what Coda hands in to end the work. Its typed parts are the work, with its labels; quotes for those labels, which are optional; the statement that the work cites no source, when that is so; and the Basis and limitations.
- **Complete:** a finish is complete when it has the work and the Basis and limitations, every citation in it is valid, and it has at least one citation or states that the work cites no source.
- **Coda's description:** Coda's short account of a source, used only when nothing from the source itself can be shown.
- **Panel:** the side panel that shows a source as a readable record, beside the work.
- **Layout:** a saved arrangement for one source and one record type. It places the record's fields and never holds a value.
- **Basis and limitations:** Coda's account of what the work rests on and what is missing, uncertain or assumed.
- **Sources:** the control that lists each cited source once.

## States and steps

One piece of work, as Coda writes it:

| From | Step | To |
| --- | --- | --- |
| Working | Coda reads a source | Working: that version has a label, and what Coda read is delivered |
| Working | Coda starts from a fresh context | Working: nothing read before is delivered until Coda reads it again |
| Working | Coda hands in a complete finish | Finished: the work is saved with its citations, or with none, and with its Basis |
| Working | Coda hands in a finish that is not complete, and it lacks the work or the Basis, or fewer than two finishes of this work were refused | Working: the finish is refused, nothing is saved, and Coda is told what is wrong |
| Working | Coda hands in a finish that has the work and the Basis but is not complete, after two finishes of this work were refused | Finished: the work is saved with its valid citations and its Basis, and each citation that is not valid is dropped |
| Working | the work is stopped, is cut off, or can't be finished | Not finished: no citations and no Basis are saved |
| Not finished | the work continues | Working |

The panel, for one reader:

| From | Step | To |
| --- | --- | --- |
| Closed | you choose a citation | Loading that citation |
| Loading | the source loads, and its place is found | Shown: the version Coda read, at the place, highlighted |
| Loading | the source loads, and the citation has no place | Shown: the version Coda read, without highlighting |
| Loading | the source loads, and its place can't be found in the readable record | Shown: the same version without highlighting, with a short notice |
| Loading | the source can't load | Couldn't load, with Retry |
| Loading | you may not access the patient, or the content is withheld after a wrong-patient correction | Unavailable, with the reason |
| Loading | an answer arrives for a citation you chose earlier | Loading: the answer is dropped |
| Couldn't load | you press Retry | Loading the same citation |
| Shown | your access to the patient ends, or the content is withheld after a wrong-patient correction | Unavailable, with the reason |
| Loading, Shown, Couldn't load or Unavailable | you choose another citation | Loading that citation |
| Loading, Shown, Couldn't load or Unavailable | you close the panel | Closed |

How the panel reads a record:

| From | Step | To |
| --- | --- | --- |
| A record to show | the source supplies its own readable text | The panel shows that text |
| A record to show | there is no such text, and a saved layout fits the source and record type | The panel shows the record arranged by the layout |
| A record to show | there is neither | The panel shows the general view: every field under its name |
| A record type with no layout | Coda makes a layout, and it passes its checks | The layout is saved for that source and record type |
| A record type with no layout | the layout fails a check, or it can't be made | No layout is saved, and the general view is used |

## Rules

1. Every citation in finished work names a version the harness gave Coda in that work. A citation's place, when it has one, is in text that was delivered when Coda handed in the finish.
2. Finished work has exactly one Basis and limitations.
3. Work is saved from a finish that is not complete only after two finishes of that work were refused.
4. Work that was stopped, was cut off or couldn't be finished has no citations and no Basis.
5. A refused finish saves nothing. A finish that lacks the work or the Basis is always refused, and any other finish that is not complete is refused at most twice for one piece of work.
6. The panel shows a source's content only while you may access its patient and the content is not withheld.
7. The panel shows only the citation you chose last: an answer for an earlier citation never replaces it.
8. When the panel shows a source, it is the version its citation names, and a highlight is at the place its citation names and nowhere else.

These rules rest on the code, its records and the screen, not on the order of steps, so the model doesn't check them:

- The label is the only part of a citation that is read from the model's text. Places, descriptions and links never come from it, and code never decides anything from the prose of the work or of the Basis.
- A quote only adds a place to its citation. A quote that is missing, or that is not in the delivered text, leaves the citation without a place. It never makes a finish incomplete, and its words are never shown.
- Quotes for a label are given in the order the label is written. A label written once takes every quote given for it. One quote for a label serves every place it is written. With any other count, every place the label is written takes every quote given for it. The Basis and limitations takes no quotes.
- A citation that was dropped never shows. The finish that held it stays in the run's record.
- A source keeps one number through a piece of work, however many claims cite it, and the Basis and limitations uses the same numbers. Each citation still opens the version and the place it names, also when several citations share a source.
- A citation opens the version Coda read, also after the source has changed. The panel shows the source as of the work, with that date, and doesn't mark that a newer version exists.
- Pointing at a citation shows what the source is, from the record itself, and the cited place when it has one: the passage quoted, the fields it covers with their values, or the page.
- Coda's description shows only when nothing from the source itself can be shown. It is marked as prepared by Coda, with the date Coda read the source, so Coda's words and the source's words never look alike.
- The panel never picks a place because its text is equal to the quote.
- Every value the panel shows is copied from the record. No value comes from the model or from a layout.
- Nothing in the record is dropped: a field no layout places still shows, under its source's name for it.
- A part of a record reads as not supplied only when the source says it is absent.
- A source reads as unavailable only when it can't be read or shown, or is withheld, never because the model said so.
- The panel shows the record's own status beside its kind and date, and an inactive, refuted, entered-in-error or preliminary record says so before its statement.
- A structured record's raw data stays under Technical details and shows nowhere else.
- The live link into the EHR comes from the source's adapter and shows only when the adapter supplies one.
- Sources lists each cited source once. Work that cites nothing shows no Sources control.
- Basis and limitations sits collapsed beside Sources and opens in place. It says what the work rests on and what could change it, and it is never a generic disclaimer.
- Basis and limitations is never part of a message to a patient.

## Outside this spec

What reaches the model, when a fresh context starts, and how the harness gives labels follow [harness.md](../../../intent/harness.md#guarantees). How records are read from each source and filed follows [physician.md](../../../intent/physician.md). How an answer ends, what continues it, and what a stopped or partial answer shows follow [chat](../chat/README.md). Records that arrive while Coda is writing the work are not in this spec: a citation is as of what Coda read.
