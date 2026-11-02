# Coda Agents

This is what we're building for physicians: what Coda Agents is, why it exists, and how it should behave. It describes the product as we intend it today. Open questions live in DECISIONS.md, and git keeps the history.

## What Coda Agents is

Coda Agents is a team of health agents that works for a physician. It isn't a copilot that waits to be asked. It behaves like a great resident team: it keeps each patient's story current, carries the work that happens between visits, and brings the physician in only where their judgment, an exam, a procedure or the relationship actually needs them.

Our customer is the physician-owned membership practice, concierge or direct primary care. These physicians know their patients well and want to stay close to them, but the work between visits (new results, patient messages, follow-through on care plans, getting ready for the next visit) eats their time. The wedge is tracking patients between visits and carrying follow-through to completion, with visit preparation as a core promise. Follow-through and help with the patient inbox are sold as one product.

The EHR stays the system of record and Coda is the system of action. Coda reads the EHR, the practice's patient mail, outside records, a connected patient's Duet data and the clinical literature, and connects the dots across all of it. The EHR is one input among several. In the MVP Coda keeps a dated working copy of what it reads, with the EHR as the source of truth.

Coda carries five jobs for the physician: preparing for encounters, reviewing new information, carrying follow-through, reconsidering the broader trajectory, and investigating the physician's questions. Those jobs are where Coda focuses, and they never limit what the model may do.

## The point

The measure of success is more patient relationship without more physician work. Concretely, physician-touched items per 100 patients per week has to fall, while the care patients need still happens. Every design choice should be read against that number. If a feature makes the physician touch more things, it needs a very good reason.

## Principles

Coda's shared principles (PRINCIPLES.md) govern this spec. These are the ones specific to the physician product.

1. **A team, not a copilot.** Coda reviews what changed, carries what is due, and brings the physician in only when their judgment helps. Within its authority it is extremely proactive.
2. **The physician holds clinical authority.** The physician decides diagnosis, treatment and medication. Nothing reaches a patient without the physician, even in an emergency. Every message to a patient needs the physician's approval; updates Coda notices, such as a documented care decision reaching the patient's Duet, need none.
3. **Every answer comes from evidence Coda actually read,** with a citation the physician can open. When evidence is missing or unreadable, Coda says so plainly. Every claim the product makes, such as "read", "none", "current" or "checked", traces to recorded evidence.
4. **Design for the patient who does nothing.** Coda notices what changed and carries each episode to the physician's decision and through follow-through without the patient lifting a finger.
5. **Attention stays salient.** Not everything becomes a concern. Quiet work that needs nothing from the physician finishes without a receipt and stays inspectable from the patient's history and the Updates log.
6. **Ambient.** The physician doesn't have to ask Coda to start ordinary work. Preparation, reviews and follow-ups start from the schedule, the clock, scheduled pulls of each source and arriving patient mail.
7. **Build for every practice.** What differs between practices is configuration or a declared adapter, never a constant or a branch written for one practice.
8. **Silence that looks like health is a failure.** If something is behind, stuck or degraded, someone who can act on it sees it, with the reason. Recovery is the system's job, and retries never become the physician's workflow.
9. **Live where you work; history as it was.** What the physician is looking at updates live. Anything already said or sent keeps the version it used and is marked when something newer exists.
10. **Messages stay in the practice's channels.** Gmail and Spruce remain the inbox, and Coda has no inbox of its own.
11. **Never anchor on FHIR or any one API.** Every source comes through its own adapter.
12. **Fail closed on access, consent and patient data; fail gracefully everywhere else.** A small failure never closes a whole screen, and a list never reads as complete when something is missing.
13. **The shared UI principles apply** (PRINCIPLES.md U1–U9). In particular, a label or control exists only because a real state or capability exists.

## The concepts

**Coda.** The team of agents. It investigates, synthesizes and recommends instead of pushing routine judgment back to the physician.

**Today.** The physician's to-do list. It shows the most salient things: what needs their attention, what's urgent, what's wrong. It is one flat list across patients, one row per thing, each naming its patient, ordered by urgency (now, then today) and then by how long it has waited. Coda decides what goes on Today, what goes to Updates and what stays on the patient page. Three things are fixed regardless of that judgment: only people with access see a patient's item, nothing broken, behind or unhandled is silent to someone who can act, and nothing reaches a patient without the physician's approval.

Things Coda typically puts on Today:

- a concern, cleared with Done;
- one row per patient message, carrying the drafted reply and any concern about that message (Review reply);
- a message waiting to be matched to a patient, on the messaging manager's Today, and an urgent-looking unmatched message as Now for everyone who sees it;
- a message Coda can't read, linking to it in Gmail or Spruce until it's read or marked Done;
- a visit nudge, when Coda sees something new before a visit;
- work Coda can't restart that left something unhandled, one row per patient, cleared with Done;
- running work the physician would recognize, such as a reply to a patient's message or something they started;
- a disconnected messaging account, so Today never claims nothing needs the physician;
- a For my doctor summary a connected patient sent, when Coda judges a physician should see it;
- one line for the practice when its whole records source is down.

The day's schedule sits in a collapsible pane beside the list. Below the rows, one collapsed line summarizes everything else Coda did that day ("Coda also reviewed 9 patients today"), and opening it shows that day's Updates. A Now item stays at the top of every page until it has been opened once. Opening a row marks it seen and never removes it; a row leaves when the physician acts (Done, No reply needed, or a reply Coda sees them send) or Coda resolves it. A seen row that changes comes back as new and says what changed.

**Updates.** The log of everything Coda did across the physician's patients, which the physician never has to maintain: what it wrote, quiet reviews that needed nothing (grouped under a heading like "Reviewed, nothing for you"), concerns it closed with a one-line reason, follow-up changes, work that ended with nothing to do and why, and replies it drafted. Earlier days are one click back. Opening an update lands on the exact work and evidence behind it. A reply the practice sent itself gets no Updates row; the patient's history records that the practice answered.

**Concern.** One thing that needs the physician's judgment, written by Coda for one patient and marked now or today. Unrelated concerns are separate items the physician clears independently, and Coda decides when concerns belong together. A concern tied to an upcoming visit ends with the visit, except a Now concern, which stays until someone opens it. Coda's own unprompted findings reach the physician only as concerns cleared with Done. Done clears a concern for every physician sharing the patient; it doesn't end a follow-up or say the issue is resolved. Coda can also retire a concern, with a one-line reason in the patient's history and in Updates.

**Patient page.** One place per patient that answers what's happening, what Coda is doing, what it's following up on and what needs the physician. Its top shows that patient's Today rows with the same actions. Below that are the patient summary (open by default), follow-ups, conversations, messages, past work and the routine work Coda is doing now. It says "Records as of {time}" from the last good pull, and says so when records couldn't update.

**Patient memory.** One current picture of each patient, built from records, results, messages and the physician's conversations with Coda, and updated as things change. Coda keeps its own revisable working understanding of the patient, the operating index, which is never shown to the physician and never the source of truth for medications. The physician sees the patient summary instead. What the physician says in chat becomes a dated part of memory. Past work is searchable history, and memory search is semantic. Coda rewrites the summary when a change matters to the physician, and the periodic review rewrites it when anything it cites has changed.

**Initial understanding.** The first time Coda sees a patient, it does the work to understand them and builds the operating index, including finding the patient's other health systems. Later sources update that understanding instead of restarting it, and a new EHR source never creates a second patient relationship. When a patient's only source is their Duet, Coda builds a deliberately limited first understanding from it.

**Patient review.** Coda's look at a patient. A review starts when a scheduled pull finds something new, patient mail arrives, a follow-up comes due, a visit approaches, the periodic review comes around or the physician asks. One event starts one review, and a message and a result found together share a review. If records change before it finishes, it looks again. It may end quietly. A connected patient's Duet data is evidence like any other, labeled with where it came from ("from Epic, via the patient's Duet"), and plan Check-in answers from Duet feed reviews without ever becoming alerts.

**Follow-up.** A responsibility Coda carries for a patient when some time should pass before checking in. It has a due time and lives on the patient page; Today only gets what needs the physician. The physician creates, changes, stops, resumes, reviews now, completes and cancels follow-ups directly or in conversation, and Coda's own work may create, revise or complete them too.

**Briefing.** Coda's preparation for a visit: the patient's meaningful trajectory, what's new, what's unresolved and the decisions the visit can settle, every point opening its source. A briefing belongs to one visit time. A finished briefing doesn't mean its recommendations were accepted.

**Patient message and reply.** A message arrives in Gmail or Spruce, is linked to one patient, and wakes that patient's review. If a reply would help, Coda proposes one grounded in the chart and the whole conversation. The physician edits it or asks Coda to revise it, then copies it and sends it themselves.

**Conversation.** The physician's chat with Coda about one patient, distinct from a patient's message to the physician. Away from a patient, the sidebar lists recent conversations; a patient's page keeps that patient's conversations.

**Run steps.** While Coda works, the physician sees steps that come only from real tool calls, worded for a physician and never as tool names or internal vocabulary. They replay later from the patient's history and never become Today items or notifications. Physicians see steps, not the model's reasoning. Pulsing dots show running work.

**Citations.** A citation opens the exact source Coda read, with a short dated preview Coda wrote, the passage for that claim highlighted, and a live link into the EHR when one exists. One source can back several claims, and each citation opens its own passage. Citations can point to records, literature, wearables, patient messages and other sources. Every record Coda read gets its preview; "unavailable" only applies to a source Coda couldn't read at all.

**Basis and limitations.** Every response carries a basis-and-limitations section, collapsed by default and written separately from the response, beside Sources. It's there on every response for regulatory reasons. Important uncertainty still belongs in the response itself.

**Stop and Resume.** Stop holds exactly the work it names, across restarts, until Resume. Infrastructure interruptions aren't Stops; they recover on their own.

## Journeys

### P1. Ask about a patient, with evidence

1. The physician finds a patient with Find patient, a server-side roster search that respects access and is shared by everything that searches patients, and opens the patient page. If search ever falls back to keywords, someone is told.
2. The patient page shows identity with DOB and MRN labeled (whenever the source supplies them), the patient summary and when records were last refreshed.
3. The physician writes a question, or dictates it: hold the shortcut or click the mic, release to transcribe, and the text lands in the box for editing. Nothing sends on its own. Asking from the patient page starts a new conversation about that patient.
4. Coda reads the chart, prior conversations, its understanding of the patient and, when useful, the clinical literature. Whether to consult literature is Coda's judgment, and chart answers work when the literature service is down. Coda sees how old its copy of the records is and fetches fresh records when it judges it needs to.
5. While Coda works, steps appear live. The finished answer arrives whole.
6. A reworded question still finds the evidence, and one failed read doesn't fail the whole answer.
7. The answer cites evidence beside each claim and ends with one Sources control and a collapsed Basis and limitations. Missing or unreadable evidence is said plainly, and unavailable literature reads differently from literature that found nothing.
8. The physician opens a citation and the source opens in a side panel beside the answer. If the passage can't be recovered, the same source shows without highlighting and with a short notice.
9. Later, the physician reopens the answer and its citations and finds them as they were.

Along the way:

- **Steer.** Sending while Coda answers changes the answer's direction at once. Nothing typed is ever lost.
- **Queue.** Send next queues a message. Queued messages stay visible, editable and deletable, and one can be sent right away while no answer is running.
- **Stop.** Stop really stops, and queued text returns to the composer. The stopped answer stays in the transcript marked stopped and keeps its Resume until the physician sends something else.
- **News mid-answer.** Records or messages that arrive before the answer finishes are taken in.
- **News after the answer.** A finished answer stays as it was said and is marked with Update when newer records arrive.
- **Saving.** Drafts save quietly. Only a draft that can't be kept says "Not saved" with Retry, and a failed send keeps its text in the composer.
- **Lost receipt.** A command changes its item only once accepted. A lost receipt shows as pending, Coda confirms it on its own and never sends twice, and if it still can't confirm, the item says so with Check again.
- **Restart.** After a restart the physician finds conversations, drafts, history and feedback exactly as they left them, and interrupted work recovers without duplicate effects.
- **Archive.** Archiving a conversation offers Undo. It's allowed only with no running or queued work, and it never clears attention or ends follow-ups. Sending, Resume, Try again and Update each bring an archived conversation back.
- **Correction.** See K1.

### P2. A scheduled pull finds a new result

1. Coda pulls each patient's sources on a schedule. A new result is found by the next pull, and a full review starts only when something changed, so a quiet day costs little.
2. A pull finds a new result and a review starts. A patient's work runs side by side, ordered only by real dependencies.
3. Coda pulls up the patient, the last plan and what changed. Routine reviews stay off Today, and the patient page shows what Coda is doing. Stopping a review stops only that review, which waits on the patient page with Resume while new results still get their own reviews.
4. The review ends in one or more outcomes: a concern on Today, a proposed reply when a message is involved, a follow-up created, revised or completed, a concern retired with its reason, or nothing to do, logged in Updates with its reason.
5. The physician opens Today and sees what changed and what needs them, and nothing that doesn't. Opening a row or an update lands on the exact work and evidence behind it.
6. The physician clears one concern with Done and keeps the others.
7. Over time the physician trusts that new results, due follow-ups and scheduled reviews were handled, including when nothing needed them.

Along the way:

- **Shared patient.** When two physicians share a patient, Done clears the concern for both, and each keeps their own drafts and read state.
- **Review can't finish.** It shows Couldn't finish with Try again on the patient page and the work itself, never on Today. If Coda can't restart it and something is left unhandled, it waits on Today with Done, and Done hands its unreviewed results to the next review.
- **Nothing left to review.** If a review has nothing left to do (the reply was handled, the message moved), Coda records why and shows no row.
- **A source can't update.** Coda keeps working from the last good records, and the patient page and any briefing say "Records as of …, couldn't update". "As of" is when Coda last read the source successfully. A missed scheduled fetch shows as couldn't update; there's no age threshold. If the practice's whole source is down, Today carries one line for the practice instead of a row per patient.
- **Part of Today can't load.** Today shows what loaded plus one quiet line with Retry, and an item that fails to open says "Couldn't load this" with Retry and keeps older content.
- **A lab is corrected.** The review updates, keeps the physician's edits and unrelated history, and retires any reply built on the old value.
- **Restarts and duplicates.** No accepted work is lost and no review runs twice.
- **Urgent finding.** Coda marks the concern Now. It leads Today and stays at the top of every page until opened once, possibly with a banner. There's no alarm, and Coda takes no action beyond telling the physician.

### P3. A patient message is linked, a reply is proposed, revised and handed off

Connecting a channel:

1. Each physician connects their own Gmail with read-only access; Spruce connects once per practice. Each physician sees only conversations for patients they may access.
2. Gmail watches the whole mailbox by default, Promotions included. Labels are an optional restriction, and Spam and Trash stay out unless included.
3. Coda starts with new messages and offers an explicit option to review a backlog.
4. Catching up and temporary sync problems are never shown. Only a revoked grant or an administrator's block asks the physician to Reconnect, with the reason.
5. Disconnecting and reconnecting loses and duplicates nothing. A completed disconnect deletes messages that were never accepted into a patient's record, and reply drafts; accepted evidence stays under the record's policy.

A message arrives:

6. A message arrives. Coda handles mail from patients; mail from colleagues or insurers about a patient is outside the MVP. The model decides whether mail is patient mail, with no brittle filters.
7. A conversation or sender already confirmed for one patient routes on its own, and so does a Spruce conversation whose one outside participant is saved as the contact of exactly one patient, linked to exactly one EHR record that matches exactly one Coda patient. Every other message gets Coda's suggestion and waits for a person to confirm. Matching always ends with a suggestion or a person deciding, and it never hangs.
8. One conversation belongs to one patient.
9. A message waiting for a match goes to the messaging manager and to a practice-wide Messages to match list any physician can match from. It shows the suggested patient and alternatives, and Confirm names the patient. Model scores never appear as confidence. The person confirms, picks another patient or leaves it unlinked.
10. An unmatched message that looks urgent can be marked so, and its row leads Today as Now for everyone who sees it.
11. Once linked, the message joins the patient's record and wakes their review. The whole conversation, including replies the practice sent itself, becomes evidence Coda can cite.

A reply is proposed:

12. The review decides whether a reply would help. Acknowledgments and resolved conversations get no draft and no Today row.
13. A proposed reply shows on Today as one row for the message: the start of the patient's words, Drafted a reply (with Flagged when Coda also has a concern), and Review reply. The row leaves when both are handled.
14. The physician opens the reply in place and sees the patient's unanswered messages with attachments, then what Coda flagged, then the proposed reply, with Basis and Sources describing Coda's work behind it.
15. The patient's words are never changed, only formatted, with unsafe content stripped. Readable attachments are used, and unreadable ones are named.

Revise:

16. The physician edits the reply directly, or asks Coda to revise it in place, and Restore my version brings their own text back. Anything new that arrives before the draft is shown is taken in first, so the physician only ever sees a current draft.
17. If the patient writes again, an untouched draft updates in place with one quiet line. If the physician already edited it, the card says the patient sent another message, and Coda's update replaces their text only if they choose it.

Hand off:

18. The physician copies the reply and sends it themselves from Gmail or Spruce (Copy reply, Open in Gmail or Spruce). Coda keeps no approval state for patient mail and never sends it in the MVP.
19. The row stays until Coda sees the sent reply. A reply the practice sent outside Coda shows on the card right away and wakes the review, which decides whether anything is left to track. The patient's history records that the practice answered.
20. In the ideal product, after the MVP, the card offers Send beside Copy, and Coda sends exactly the words the physician approved, shown as sent by Coda for the physician, with accepted, sent and delivered kept distinct.

Along the way:

- **No reply needed.** The physician closes the reply without giving a reason and gets a short Undo. A newer message's reply replaces a pending No reply needed.
- **Ask Coda to reply.** If no reply is ready, the physician asks for one and Coda always drafts one. The ask waits behind a review that's already running, and a reply from that review answers it.
- **Wrong patient.** The messaging manager uses Wrong patient? on a confirmed link (another patient, or not a patient). The fix applies going forward: the message stops affecting the wrong patient and moves to the right one, and one correction never starts two reviews. Undo within a few seconds restores the reply exactly.
- **Wrong link after use.** Until the re-review decides, anything that relied on the moved message is withheld (never left blank), then redrafted or closed. Everything Coda wrote while its memory held the moved message is marked and can't surface. The audit trail stays.
- **Message Coda can't read.** The patient goes on Today with a link to the message in Gmail or Spruce until Coda reads it or the physician marks it Done. A message Coda is holding or retrying is never invisible.
- **Urgent message.** Coda raises a Now concern on the message's row, and the draft tells the patient to call 911 or seek emergency care, for the physician to send.
- **A connected patient sends For my doctor.** Duet hands what the patient sent, including any question, to Coda Agents, where the patient's agent talks to the practice's care team agent and the practice's agents judge what the physician needs to see. If needed, it reaches Today. The physician's approved answer goes to the patient's Duet as a message from the practice in their thread, delivered by Coda exactly as approved. Any edit after approval needs approval again. This is the one message Coda delivers in the MVP.

### P4. Pre-visit preparation and briefings

1. Coda reads the schedule from the practice's EHR and prepares visits on its own; preparation never waits for someone to start it. Coda judges which visits to prepare and how far ahead.
2. The physician sees tomorrow's visits, each with its preparation status, in the schedule pane beside Today. Finished visits drop off, and on a narrow screen only the next visit shows.
3. An appointment Coda can't match to a patient stays visible with its time and "Preparation unavailable".
4. Preparation checks that the patient's records are ready before it starts. A source that can't update doesn't block it; it works from the last good records and says so. If records change before it finishes, it looks again.
5. The physician opens a briefing: trajectory, what's new, what's unresolved and the decisions the visit can settle, every point opening its source, with late or missing sources stated. The visit's card carries a short orientation Coda wrote with it.
6. A briefing reaches Today only as a visit nudge, when Coda sees something new before the visit, and the nudge ends with the visit. Nothing opens on its own.
7. Briefings stay readable from the patient page and Updates afterward.

Along the way:

- **Preparation fails.** It recovers on its own. If Coda can't restart it, it stays on Today with Done until the visit ends.
- **Visit cancelled or moved.** Unfinished preparation for that time ends at once. A finished briefing stays in history with a notice, and a moved visit is prepared fresh.
- **Visit in progress.** Preparation can still start until the appointment ends.
- **Urgent nudge.** An urgent nudge stays after the visit until someone opens it; ordinary nudges leave with the visit.
- **Later input.** A message or result that arrives later doesn't rewrite the briefing; a later review raises its own concern if needed.

### P5. A follow-up is created, comes due and runs

1. The physician asks for a follow-up in conversation or creates one directly, and Coda's own work may create one too. Every change is logged in Updates.
2. A follow-up has a due time.
3. The physician sees it among the patient's active follow-ups. Coda judges whether a due follow-up needs Today, and usually it doesn't.
4. It comes due on its own schedule, survives restarts and runs once, as a review of that patient. If something needs the physician, a concern reaches Today; otherwise it ends quietly in Updates.
5. The run may complete, revise or keep the follow-up.

Along the way:

- **Evidence arrives in a message.** That message's review handles the follow-up.
- **Review now.** The physician asks for a review of the follow-up now, and it always starts without waiting on unrelated work.
- **Stop.** A follow-up has one Stop, which also holds its running review. Resume releases both and reviews the current version.
- **Stop and discuss.** The physician stops the work and opens a conversation about it; the work stays held until Resume.
- **Change.** A change made in chat replaces the pending follow-up before the answer completes.
- **Cancel.** The control reads Cancel follow-up. Coda doesn't recreate a cancelled follow-up unless new information arrives.
- **Coda can't restart its review.** It waits on Today with Done, which offers complete, set a new date, or cancel, so the follow-up always keeps a next step.
- A follow-up is never time-critical monitoring.

### L1. Patients who do nothing

This is how we prove principle 4.

1. A multi-week world of synthetic patients and a practice runs on the real coda-rust services. Time advances, the app closes and services restart.
2. Coda checks each patient on its schedule, reviews when something changed, and runs follow-ups on their own schedules. Locally the simulation drives this on its own clock.
3. Patients never act. Coda notices what changed and carries each episode to the physician's decision and follow-through.
4. The physician sees only what needs them, and everything else is logged in Updates.
5. An undecided medication recommendation never becomes a plan, and a corrected lab retires a stale reply.
6. The run reports physician-touched items per 100 patients per week, read alongside whether necessary care happened.
7. A clinician on the team starts it locally with one command, uses the physician app, and reruns and shapes scenarios.
8. The simulation supplies the world, the events and the actors, never the model's conclusions. Evals are the jobs the product must do, run as simulations across patient types and scenarios.
9. The synthetic panel has a realistic spread of time since each patient was last seen. Each chart stops at the patient's last contact and ends at the simulation's today, with pending items and booked visits but nothing later.

### A1. Sign-in and access

Physicians sign in only through Coda Agents, with an emailed link and no password or code. Accounts are invite-only. The session is shared across the browser's tabs, signing out signs out everywhere, and another physician's link signs the current physician out first. The full sign-in behavior, with its model, lives in [sign-in](../verification/physician/sign-in/README.md).

Inside the workspace:

1. A physician sees same-clinic patients outside their panel as identity only: name, DOB and MRN, no clinical content.
2. When a physician loses access to a patient, that patient's screens close at once and anything that physician personally started stops; practice-owned work continues.
3. Every list across patients reads within the physician's access. A list that couldn't check a patient skips only that patient and says it may be missing someone.
4. Spruce conversations show only for patients the physician may access.
5. Correcting a confirmed message link belongs to the messaging manager, who is whoever connected the Gmail account or set up Spruce.
6. Any practice member can invite or discharge a roster patient from that patient's roster row. Discharge asks for confirmation and takes effect at once. The connection itself belongs to Connected care.
7. Authorized operators inspect and adjust cadence by practice or patient. Physicians don't change it, but they can ask Coda for specific scheduled jobs for a patient.
8. Authorized Coda staff may inspect submitted feedback and the minimum evidence linked to it.
9. Drafts and seen state live on the server, not in the browser.

### K1. Correcting Coda

Feedback:

1. Hovering a finished response shows feedback controls beside Basis and Sources.
2. A thumb saves the rating at once against that exact response. Optional details open with their own Save, and closing them keeps the rating. Feedback survives restarts.
3. A reply card has one feedback path, "Tell Coda what to change", and the physician's edits are feedback too.
4. Each Done on a concern is kept as feedback for later learning.

Discuss:

6. Discuss is a question to Coda about a response, separate from feedback. It opens a new chat on that topic.
7. Repeating the same Discuss click returns the same chat; a later deliberate Discuss opens a fresh one, so a long follow-up never gets crammed into one growing transcript.
8. Discuss holds running work the way Stop does, until Resume.
9. A reply has no Discuss; the physician revises it in place.

Correcting a fact or giving direction:

10. The physician says it in chat in plain words, and Coda acts on it within its authority.
11. Later answers reflect the correction, unrelated history stays, and the correction is cited to the message it came from.
12. The physician may accept, change, reject or ask Coda to reconsider. A correction becomes patient context, never approval of the earlier reading.
13. A cancelled follow-up stays cancelled unless new information arrives.

A wrong message link is fixed with Wrong patient? (P3).

## Rules

Authority and sending:

- The physician decides diagnosis, treatment and medication.
- In the MVP Coda never sends patient mail, orders, or writes back to the EHR. The one message it delivers is a physician-approved answer to a question a connected patient sent through Duet, into Duet's thread, exactly as approved; any edit needs approval again.
- Coda never sends anything to a patient without the physician, even in an emergency.
- A retrieved record that tells Coda to do something is evidence, never an instruction.
- A follow-up grants no access and no authority to message, order, schedule or write back.
- Silence, elapsed time or a finished run never proves physician approval or clinical resolution.

Access and privacy:

- Every list reads within the physician's access, and opening a source needs access to clinical content.
- Identity-only access shows name, DOB and MRN and no clinical content.
- Any signed-in physician can match a patient message; correcting a confirmed match belongs to the messaging manager.
- The chart's phone and email show only to the messaging manager while matching.
- A reply row names the channel's sender, never the linked patient, so a family member's words aren't put in the patient's name.
- The linking pass reads unlinked mail only to decide who it's from and whether it's patient mail. Patient review never reads unlinked mail.

Data and consent:

- Synthetic data only, until we decide to work with real patients.
- Real patient messages are reviewed only through model APIs covered by a BAA.
- Coda's agents may send patient-specific questions to outside clinical AI vendors, such as UpToDate's Expert AI, under a BAA. Coda's own library stays the default.
- Each record keeps its source, date and identity and is attached to the right patient.
- A record missing from a later pull is noticed and never deleted on absence alone.
- Calls are kept only as a record that a call happened; recordings are never fetched or kept. Spruce texts, voicemail transcripts and fax pages are evidence.
- A message moved off the wrong patient is marked entered in error and never deleted. None of its content stays in that patient's record anywhere, and the flags, summaries, citations and follow-ups it produced there are retracted or re-reviewed.

Evidence and honesty:

- Citations refer only to evidence the model that wrote the answer actually received.
- Any answer, reply or action about current medications or values re-reads the chart first. Memory never stands in for live chart values.
- Nothing is silently skipped. What Coda didn't read, it says, and a citation into a scan or fax points to the page.
- The library's freshness is reported as it is.
- Patient summary dates say when Coda wrote, reviewed and pulled. None of them says the patient is stable.
- No "saved" label ever shows, only "Not saved".

Safety:

- Coda isn't for emergencies, autonomous monitoring, alarms or time-critical directives.
- Coda's clinical communication follows the non-device CDS guidelines kept in its skills.
- An urgent finding creates a prominent Now concern and only communicates. Coda takes no other action.

Work integrity:

- One event starts one review, and no event is lost or reviewed twice.
- Rows and cards change when the server confirms, never optimistically.
- One bad item never blocks the rest, in every connector: one patient's failed filing never holds up other mail, and an EHR outage never holds up a new message's review.
- Background work never steals focus or opens anything on its own.
- The physician's time zone comes from their browser.
- Panel choices are remembered per physician, and cancelling dictation keeps the text.

## Where Coda's information comes from

Everything Coda reads in the MVP is read only.

- **The practice's EHR**, with Elation and Hint the two we're focused on first.
- **Hint**, through Coda's Chrome extension in the practice's own Chrome. The physician signs into Hint as usual, and the extension reads the chart's structured data in the background under that sign-in. It downloads nothing, never sees the physician's password or MFA codes, and keeps Hint's session only inside Chrome. Coda's access lasts as long as the physician's Hint sign-in. When Hint signs out, Coda keeps what it read and catches up the next time the physician signs in. Pulls run during the practice's business hours, and Coda asks the physician to reconnect only when fresh Hint data is needed and access is missing.
- **The schedule**, from the practice's EHR.
- **Patient mail** from each physician's own Gmail and the practice's Spruce.
- **Attachments**, including scans and faxes read through OCR.
- **Outside records** from other organizations through Kno2, with Coda acting as the physician's delegate. They complement the practice's chart and never replace it.
- **A connected patient's Duet data**, which lives in Coda's record for that patient and never in the EHR. Connected care owns what crosses.
- **The clinical library**, Coda's own, searched semantically and served from the cloud, always on before launch.
- **Outside clinical AI** under a BAA, such as UpToDate's Expert AI.

Every source shows when its records were last refreshed, and one source's failure never blocks the others.

Coda's Chrome extension acts only as the signed-in physician, never takes the physician's tab or focus, and never enters credentials.

## Scope

In the MVP:

- One practice using Coda Agents on the web, with patients on Duet, the two connected through Connected care.
- Journeys P1 to P5 and L1, with P1 first.
- Elation and Hint as EHRs, read only.
- Gmail and Spruce as the patient-message channels, both first-class, with Copy reply and native send.
- Delivering the physician-approved answer to a connected patient's Duet question.
- Reading attachments, including scans and faxes through OCR.
- Dictation into the composer.
- The clinical library with semantic retrieval: a documented, tag-defined PMC set that includes reviews and case reports, plus outside clinical AI under a BAA.
- Outside records through Kno2.
- A Filtered list of mail the linking pass judged not to be from a patient, with "This is a patient" to restore it, and the same through plain words in chat.
- No paid tier; everything is free.
- Done when every journey is walked on real services, hosting and real-patient readiness are in place, and we decide to go.

Not in the MVP:

- A Coda inbox of its own.
- Coda sending patient mail, ordering or writing to the EHR.
- Patient-facing voice AI.
- The visit-note scribe.
- Mail from colleagues or insurers.
- Clinical trials data in the library.
- A desktop app.

Later:

- Send beside Copy, where Coda sends exactly the approved words for the physician.
- EHR write-back, starting with a physician-approved unsigned draft note entered by computer use.
- Licensed clinical content (UpToDate, DynaMed, Wiley) once a contract lands; nothing in the MVP waits on it.
- The Chrome extension also as the minimized, ambient Coda and a way for replies to reach the native composer. Until then the web app's collapsed Coda, the orb, carries the banner and Updates.
- Coda reaching out to patients from its own findings.
- Learned physician preferences, from the physician's own words and edits only, with Undo.
- Practice-wide direction from Today, such as panel sweeps.
- Rich results in chat, such as lab tables and trend charts, every value a cited record value.
- Coda-chosen visualizations of vitals that stay with the patient's understanding.
- More patient-message channels: native EHR messaging, email and text.
- The shared patient–physician space.
- Self-serve for hundreds of practices.

## Engineering decisions

These are how we build the product above. They matter to its behavior but are engineering's to change.

**Models.** Two roles. Sol runs the physician's conversation, including its own housekeeping. Luna does background work: reviews, initial understanding, visit preparation, workers, judging and ambiguous patient linking. Background work will probably run at max effort.

**Schedules and cadence.** Proactive review of every source (the EHR, Kno2, a connected patient's Duet data) runs on scheduled jobs that pull what's new. Patient mail is handled as it arrives, and a patient joining through Connected care starts the practice's agents right away; nothing else wakes on arrival. A push, such as a Gmail notification, only wakes work, and the work always pulls current truth. Each connection fetches changes without model calls, and a model review runs only when something new arrived or a follow-up came due. A records check is a full re-pull of each patient compared with what Coda holds, starting daily. How often each schedule runs, what it costs and the per-practice or per-patient settings are engineering decisions. There's no limit on background runs in the MVP. Care-team cadence in Coda Agents is separate from patient automations in Duet.

**Memory.** The operating index is Markdown the model maintains, not a rigid schema, and its changes are auditable. Memory may become more knowledge- or graph-based after the MVP, and nothing in the MVP waits on that; any change to what is kept or shared is a product decision.

**Runs.**

- Interrupted work resumes without duplicating effects, and a retry reuses work that already finished.
- A run ends exactly once, whether it completes, is cancelled, shuts down, crashes or gets no answer.
- A failed preparation, review or initial understanding retries on its own before it counts as can't restart.
- Recovery never alerts the physician.
- When context fills, the model writes its own durable working note before older context is released, with room kept for it. The note never replaces patient memory or source evidence.
- A refused review is recorded as a refusal with its reason, and Couldn't finish, Try again and the automatic retries read from it.
- Records that change before a run finishes reach the run, and the run keeps the exact source versions it read.
- Steering prevents stale results from being published.
- Completion is accepted only after authority and currency are checked.
- Each patient's work starts independently and runs side by side. How much runs at once is an engineering decision, and code never ranks clinical importance.

**Commands and state.** Commands are accepted or not. A lost receipt resolves without a second effect, and an ambiguous command retries only itself before reading authoritative state. Reload, reconnect and another client converge on the same server state.

**Live status.** Run steps are projected from structured tool calls and recorded facts, the same on CLI and API, and the run record stays inspectable by the team. Reasoning summaries stay in the team inspector.

**Mail.** Each Gmail or Spruce mailbox has one lease, so no message is filed twice or lost. After a rollback, a mailbox a newer release has touched stops filing and shows on the health page until an operator acts.

**Reliability.** If a startup piece can't start, the server still starts, the piece retries with backoff, and readiness reports it truthfully. Storage changes never refuse to start or wipe data, and permanent records are never rewritten to add fields. The literature service is an optional gateway, and chart answers work without it.

**Sources.** Each source has its own adapter that translates its format into Coda's common evidence and one shared source view, and new sources come as new adapters. Reads happen automatically for authorized patient work. When write-back arrives, a write needs the physician's approval of the exact text, makes exactly that change, then reopens the record and reports what the chart now shows.

**Storage.** Per-practice database topology is decided when it's needed.

**The harness.** The model owns judgment, and the harness adds tools and guardrails without restricting it.
