# Patient memory

What Coda keeps about each patient between pieces of work, how a run reads and changes it, and what holds for every file. [physician.md](../../../intent/physician.md#the-concepts) links here.

## Words

- **Memory:** the markdown files Coda keeps for one patient in one practice.
- **Summary:** the short file at the top: the patient in one line, what matters now, what is open, and links to the pages.
- **Direction:** the physician's standing instructions for this patient, close to the physician's own words.
- **Person page:** the person and the people around them: goals, preferences, family, care team.
- **Subject page:** one file for one subject, such as a condition, a symptom not yet explained, an episode, or prevention. It starts with its name, its other names and the model's word for its kind.
- **Log:** one file for each day that had activity. It holds that day's notes in order and marks where each finished rounds run started.
- **Note:** one dated, numbered entry in the log, added during a run. It can carry the physician's words, as the model quoted them.
- **Link:** `[[page-id]]` in a file or a note. It names a page by the id in its path.
- **Pointer:** words that say what a fact rests on, such as `[N4]` for note 4 or `[chart]` for the chart. A fact needs none.
- **Run:** one piece of Coda's work on the patient: a chat answer, a reply revision or patient rounds.
- **Edit:** one change a run asks for in a file: create it, replace text in it, append to it, or delete it.
- **Patient rounds:** Coda's regular pass over one patient's memory. It reads the new notes and the chart, and corrects, shortens and links the summary, the direction and the pages.
- **Rounds mark:** the record that a rounds run finished. It holds when the run started and when it finished.
- **Version:** the saved text of a file after one write.

## States and steps

A note:

| From | Step | To |
| --- | --- | --- |
| None | a run adds a note | Saved in that day's log, with its number, its time, its run, its physician and its message set by code |
| Saved | the same call is repeated | Saved: nothing is added |

A file:

| From | Step | To |
| --- | --- | --- |
| None | a run creates the file, or appends to it | Version 1 |
| Version n | a write with an edit that applies | Version n+1, and version n stays readable |
| Version n | a write where no edit applies or changes anything | Version n: nothing is saved, and the result names each edit and why |
| Version n | a run deletes the file, or a replace leaves it with no words | Deleted at version n+1: it is not listed or searched, and every earlier version stays readable |
| Deleted | a run creates the file, or appends to it | The next version |

A rounds run:

| From | Step | To |
| --- | --- | --- |
| None | patient rounds start for the patient | Running, opened with the notes saved since the last finished run started |
| Running | the run finishes | Finished: one rounds mark is saved |
| Running | the run stops early | Stopped: no mark is saved, what it wrote stays, and its notes stay new for the next run |

## Rules

1. A saved note is never changed or removed.
2. A note's number, time, run, physician and message are set by code, never by the model.
3. Every write that changes the summary, the direction or a page makes a new version, and every earlier version stays readable.
4. Memory is read and written only for the practice and patient it belongs to.

These rest on the code and its records, not on the order of steps:

- Every run can change memory. A chat answer, a reply revision and patient rounds read with the same tools and change a file with the same edit. Patient rounds are the regular pass, not the only writer.
- What a physician says does not start rounds. Coda is told to add a note when it learns something later work should know and could not look up in the chart, and to change a file at once when the file is wrong or is missing something that matters now.
- What memory keeps, what gets a page and how a page is laid out are the model's judgment. Memory is for eight things: the patient in one line; problems; what is in motion; why each medication was started, changed or stopped; prevention and monitoring; the person and their people; the physician's direction and corrections; and what happened, by date. Code checks none of them.
- A file is the markdown text the model wrote. Code stores it exactly and gives no meaning to its headings, bullets, links or pointers. It searches the text and finds the text a replace names, and nothing more.
- A file exists when its text has words. Text with no words is saved as no text, so a replace that leaves no words deletes the file and keeps no leftover spacing.
- The chart owns current medications, doses and values. Coda is told that memory can be behind the chart, and that memory holds what was worked out, said or decided, and dated history that takes several records to assemble.
- When the physician and the chart disagree, Coda follows the physician. Coda is told to state both in memory, each with its date. In patient rounds it is also told never to write that the chart changed.
- In patient rounds Coda is told to keep what a physician said as what the physician said, and to check a fact against the chart before it writes it as what the chart shows.
- A write saves what it can. An edit that cannot apply, or that would change nothing, is left out, and the result names it and why. No write is refused whole for one edit.
- A replace names text that is in the file exactly, in one place, and changes only that text.
- Writes for one patient apply one after the other, each to the file as it stands, so a replace does not write over another run's change to its text.
- An append starts on a new line. After a delete, a read of the path says there is no file, and an earlier version reads by its number.
- Every write tells the writer the file's size, and the list gives the size of the summary, the direction and each page.
- A page's id in a path is taken in its normal form: lowercase, with one hyphen between words.
- Search matches words on a line in every file, and a result names its file and line. A page's other names are in its header, so a search finds the page by them, and a search for a page's id finds every link to it.
- Code makes the log from the saved notes and rounds marks, and no run edits it. A mark sits in its day at the time its run started. A log day has no versions.
- A conversation starts with the summary, the direction and the notes saved since the last finished rounds run started, or all the notes when none has finished. A patient with none of them gets no message. So what the physician said holds for the next conversation, and a note saved while rounds ran is not missed.
- That memory loads once, as one message at the top of the conversation, and every later run in the conversation sends the same text. A reply and its revisions are one conversation. Memory saved later reaches the next conversation, and a run reads it with the memory tools.
- The summary loads up to a set budget, cut at a line end. The notes load whole, the newest first, up to a set budget. Past a budget, a line names the file that holds the rest. The direction loads whole. The summary and the direction each show the day they were saved.
- The physician is shown the summary and the direction. The pages and the log are Coda's working files.
- A saved note or a memory edit shows the physician one step. A memory read shows none.
- A physician's own edit of the summary or the direction is saved as a note in their words. Patient rounds fold it in.

## Outside this spec

How a run starts, recovers, keeps its context and ends, and how a started call is answered after a restart, follow the [harness's guarantees](../../../intent/harness.md#guarantees). How a step shows in a conversation follows [chat](../chat/README.md). The patient page, where the physician reads and edits the summary and the direction, follows [physician.md](../../../intent/physician.md#the-concepts), and so do follow-ups and initial understanding. When patient rounds run, rounds reading the conversations themselves, the size of each budget, what happens to memory after a wrong-patient correction, and search across everything Coda has about a patient are not in this spec.
