# Chords

This is what we're building for physicians: what Chords is, why it exists, and how it should behave. It describes the product as we intend it today. Git keeps the history.

## What Chords is

Chords is a team of health agents that works for a physician. It isn't a copilot that waits to be asked. It behaves like a great resident team: it keeps each patient's story current, carries the work that happens between visits, and brings the physician in only where their judgment, an exam, a procedure or the relationship actually needs them.

Our customer is the physician-owned membership practice, concierge or direct primary care. These physicians know their patients well and want to stay close to them, but the work between visits (new results, patient messages, follow-through on care plans, getting ready for the next visit) eats their time. The wedge is tracking patients between visits and carrying follow-through to completion, with visit preparation as a core promise. Follow-through and help with the patient inbox are sold as one product.

The EHR stays the system of record and Coda is the system of action. Coda reads the EHR, the practice's patient mail, a connected patient's Duet data and the clinical literature, and connects the dots across all of it. The EHR is one input among several. In the MVP Coda keeps a dated working copy of what it reads, with the EHR as the source of truth.

Coda carries five jobs for the physician: preparing for encounters, reviewing new information, carrying follow-through, reconsidering the broader trajectory, and investigating the physician's questions. Those jobs are where Coda focuses, and they never limit what the model may do.

## The point

The measure of success is more patient relationship without more physician work. Concretely, physician-touched items per 100 patients per week has to fall, while the care patients need still happens. Every design choice should be read against that number. If a feature makes the physician touch more things, it needs a very good reason.

## Principles

Coda's shared principles (README.md) govern this spec. These are the ones specific to the physician product.

1. **A team, not a copilot.** Coda reviews what changed, carries what is due, and brings the physician in only when their judgment helps. Within its authority it is extremely proactive.
2. **The physician holds clinical authority.** The physician decides diagnosis, treatment and medication. Nothing reaches a patient without the physician, even in an emergency. Every message to a patient needs the physician's approval; updates Coda notices, such as a documented care decision reaching the patient's Duet, need none.
3. **Every answer comes from evidence Coda actually read,** with a citation the physician can open. When evidence is missing or unreadable, Coda says so plainly. Every claim the product makes, such as "read", "none", "current" or "checked", traces to recorded evidence.
4. **Design for the patient who does nothing.** Coda notices what changed and carries each episode to the physician's decision and through follow-through without the patient lifting a finger.
5. **Attention stays salient.** Not everything becomes a concern. Quiet work that needs nothing from the physician finishes without a receipt and stays inspectable from the patient's history and the Updates log.
6. **Ambient.** The physician doesn't have to ask Coda to start ordinary work. Preparation, reviews and follow-ups start from the schedule, the clock, scheduled pulls of each source and arriving patient mail.
7. **Build for every practice.** What differs between practices is configuration or a declared adapter, never a constant or a branch written for one practice.
8. **Silence that looks like health is a failure.** If something is behind, stuck or degraded, someone who can act on it sees it, with the reason. Recovery is the system's job, and retries never become the physician's workflow.
9. **Live where you work; history as it was.** What the physician is looking at updates live. Anything already said or sent keeps the version it used and is marked when something newer exists.
10. **Messages stay in the practice’s channels.** For the MVP, Spruce is the practice inbox. The physician reviews Spruce messages in Coda and sends replies in the original Spruce conversation. Coda has no inbox of its own.
11. **Never anchor on FHIR or any one API.** Every source comes through its own adapter.
12. **Fail closed on access, consent and patient data; fail gracefully everywhere else.** A small failure never closes a whole screen, and a list never reads as complete when something is missing.
13. **The shared UI principles apply** (README.md U1–U9). In particular, a label or control exists only because a real state or capability exists.

## The concepts

**Coda.** The team of agents. It investigates, synthesizes and recommends instead of pushing routine judgment back to the physician.

**Roster.** The practice's patients, as its EHR lists them. A patient with a live Duet connection is active, and Coda works on active patients first. A patient in the EHR with no connection stays on the roster, to the side, as not active. How the first practice's roster is filled and kept current is not decided yet.

**Today.** The physician's to-do list. It shows the most salient things: what needs their attention, what's urgent, what's wrong. It is one flat list across patients, one row per thing, each naming its patient, ordered by urgency (now, then today) and then by how long it has waited. Coda decides what goes on Today, what goes to Updates and what stays on the patient page. Three things are fixed regardless of that judgment: only people with access see a patient's item, nothing broken, behind or unhandled is silent to someone who can act, and nothing reaches a patient without the physician's approval.

Things Coda typically puts on Today:

- a concern, cleared with Done;
- one row per patient conversation with a reply ready, carrying the drafted reply and any concern about its messages (Review reply);
- a message waiting to be matched to a patient, and an urgent-looking unmatched message as Now for everyone who sees it;
- a message Coda can't read, linking to it in Spruce until it's read or marked Done;
- a visit nudge, when Coda sees something new before a visit;
- work Coda can't restart that left something unhandled, one row per patient, cleared with Done;
- running work the physician would recognize, such as a reply to a patient's message or something they started;
- a disconnected messaging account, so Today never claims nothing needs the physician;
- what a connected patient sent from For my doctor, when Coda judges a physician should see it;
- one line for the practice when its whole records source is down.

The day's schedule sits in a collapsible pane beside the list. Below the rows, one collapsed line summarizes everything else Coda did that day ("Coda also reviewed 9 patients today"), and opening it shows that day's Updates. A Now item stays at the top of every page until it has been opened once. Opening a row marks it seen and never removes it; a row leaves when the physician acts (Done, No reply needed, or a reply Coda sees them send) or Coda resolves it. A seen row that changes comes back as new and says what changed.

**Updates.** The log of everything Coda did across the physician's patients, which the physician never has to maintain: what it wrote, quiet reviews that needed nothing (grouped under a heading like "Reviewed, nothing for you"), concerns it closed with a one-line reason, follow-up changes, work that ended with nothing to do and why, and replies it drafted. Earlier days are one click back. Opening an update lands on the exact work and evidence behind it. A reply the practice sent itself gets no Updates row; the patient's history records that the practice answered.

**Concern.** One thing that needs the physician's judgment, written by Coda for one patient and marked now or today. Unrelated concerns are separate items the physician clears independently, and Coda decides when concerns belong together. A concern tied to an upcoming visit ends with the visit, except a Now concern, which stays until someone opens it. Coda's own unprompted findings reach the physician only as concerns cleared with Done. Done clears a concern for every physician sharing the patient; it doesn't end a follow-up or say the issue is resolved. Coda can also retire a concern, with a one-line reason in the patient's history and in Updates.

**Patient page.** One place per patient that answers what's happening, what Coda is doing, what it's following up on and what needs the physician. Its top shows that patient's Today rows with the same actions. Below that are the patient summary (open by default), follow-ups, conversations, messages, past work and the routine work Coda is doing now. It says "Records as of {time}" from the last good pull, and says so when records couldn't update.

**Patient memory.** One current picture of each patient, built from records, results, messages and the physician's conversations with Coda, and updated as things change. Coda keeps its own revisable working understanding of the patient, the operating index, which is never shown to the physician and never the source of truth for medications. The physician sees the patient summary instead. What the physician says in chat becomes a dated part of memory. Past work is searchable history, and memory search is semantic. Coda rewrites the summary when a change matters to the physician, and the periodic review rewrites it when anything it cites has changed.

**Initial understanding.** The first time Coda sees a patient, it does the work to understand them and builds the operating index, including finding the patient's other health systems. Later sources update that understanding instead of restarting it, and a new EHR source never creates a second patient relationship. When a patient's only source is their Duet, Coda builds a deliberately limited first understanding from it.

**Patient review.** Coda's look at a patient. A review starts when a scheduled pull finds something new, patient mail arrives, a follow-up comes due, a visit approaches, the periodic review comes around or the physician asks. One event starts one review, and a message and a result found together share a review. If records change before it finishes, it looks again. It may end quietly. A connected patient's Duet data is evidence like any other, labeled with where it came from ("from Epic, via the patient's Duet"), and when plan Check-ins arrive, their answers from Duet feed reviews without ever becoming alerts.

**Follow-up.** A responsibility Coda carries for a patient when some time should pass before checking in. It has a due time and lives on the patient page; Today only gets what needs the physician. The physician creates, changes, stops, resumes, reviews now, completes and cancels follow-ups directly or in conversation, and Coda's own work may create, revise or complete them too.

**Briefing.** Coda's preparation for a visit: the patient's meaningful trajectory, what's new, what's unresolved and the decisions the visit can settle, every point opening its source. A briefing belongs to one visit time. A finished briefing doesn't mean its recommendations were accepted.

**Patient message and reply.** A message arrives in Spruce, is linked to one patient, and wakes that patient's review. If a reply would help, Coda proposes one grounded in the chart and the whole conversation. The physician edits it or asks Coda to revise it, then copies it and sends it themselves.

**Conversation.** The physician's chat with Coda about one patient, distinct from a patient's message to the physician. Away from a patient, the sidebar lists recent conversations; a patient's page keeps that patient's conversations.

**Run steps.** While Coda works, the physician sees steps built from the [harness's tool-call events](harness.md#events), worded for a physician and never as tool names or internal vocabulary. They replay later from the patient's history and never become Today items or notifications. Physicians see steps, not the model's reasoning. Pulsing dots show running work.

**Citations.** A citation is a numbered mark beside a claim. Every citation names where in its source the claim rests: a passage, a field, or a page of a scan. Pointing at it shows Coda's short dated preview of the source with that passage or field quoted. Choosing it opens the exact source Coda read in a panel beside the text, at that place, highlighted, with a live link into the EHR when one exists. One source keeps one number and can back several claims, and each citation opens its own passage. Citations work the same wherever Coda's work is read: answers, reviews, reply drafts and briefings. They can point to records, literature, wearables, patient messages and other sources. Coda's words and the source's words never look alike: the preview is marked as prepared by Coda, with its date, and the record reads as the record. Every record Coda read gets its preview; "unavailable" only applies to a source Coda couldn't read at all. The panel shows the source as a readable record, never as raw data. Where the source has its own readable text, the panel shows that text. Otherwise Coda may arrange the record so it reads well, but every value shown is copied from the record and nothing in the record is dropped. A part reads as not supplied only when the source says it is absent. The raw record stays available under Technical details.

**Basis and limitations.** Every finished piece of Coda's work that a physician reads carries a basis-and-limitations section: an answer, a review, a reply draft, a briefing. Coda writes it when it finishes, separately from the work itself. It says what the work rests on and what is missing, uncertain or assumed that could change it. It is Coda's account of its own work: not a source, not private reasoning and never a generic disclaimer. It can cite evidence, with the same numbers as the work. It sits collapsed beside Sources and opens in place. It's there every time so the physician can review the basis of Coda's work for themselves and not rely on Coda alone. That is one of the conditions the FDA sets for decision support that is not a medical device. Important uncertainty still belongs in the work itself, beside what it qualifies. It is never part of a message to a patient. An answer that was stopped or cut off has none and shows no Basis control.

**Stop and Resume.** Stop holds exactly the work it names, across restarts, until Resume. Infrastructure interruptions aren't Stops; they recover on their own.

## Journeys

### P1. Ask about a patient, with evidence

The physician asks Coda about one patient and gets an answer that cites what it rests on. The full behavior and its rules live in [chat](../verification/physician/chat/README.md) and [citations](../verification/physician/citations/README.md).

1. The physician finds a patient with Find patient, a server-side roster search that respects access and is shared by everything that searches patients, and opens the patient page. If search ever falls back to keywords, someone is told.
2. The patient page shows identity with DOB and MRN labeled (whenever the source supplies them), the patient summary and when records were last refreshed.
3. The physician writes a question, or dictates it: hold the shortcut or click the mic, release to transcribe, and the text lands in the box for editing. Nothing sends on its own. Asking from the patient page starts a new conversation about that patient.
4. Coda reads the chart, prior conversations, its understanding of the patient and, when useful, the clinical literature. Whether to consult literature is Coda's judgment, and chart answers work when the literature service is down. Coda sees how old its copy of the records is and fetches fresh records when it judges it needs to. Coda discovers records in the patient workspace on demand and opens the exact revision behind a chart claim. A large read identifies the passage delivered and where to continue in that revision.
5. While Coda works, steps appear live: each names something Coda did, such as a record it read or a search it ran, and how it ended. When the answer arrives, the steps fold into one line that still opens. The finished answer arrives whole.
6. A reworded question still finds the evidence, and one failed read doesn't fail the whole answer.
7. The answer cites evidence beside each claim and ends with one Sources control and a collapsed Basis and limitations. Missing or unreadable evidence is said plainly, and unavailable literature reads differently from literature that found nothing.
8. The physician opens a citation and the source opens in a side panel beside the answer. If the passage can't be recovered, the same source shows without highlighting and with a short notice.
9. Later, the physician reopens the answer and its citations and finds them as they were.

Along the way:

- **Steer.** While Coda answers, the composer offers Send now and Send next. Send now, also Enter, [steers](harness.md#guarantees) the answer at once. Shift+Enter makes a new line. Nothing typed is ever lost.
- **Queue.** Send next, also Command+Enter, or Control+Enter on Windows, queues a message. Coda keeps queued messages, not the browser, so they survive a closed tab and show on every device. They stay visible under the answer, and each can be edited, deleted or sent now; saving an edit reads Update. When the answer ends, Coda answers them together in one reply. If the answer can't finish, they stay queued: Send now sends one, and Try again continues the answer first.
- **Stop.** Stop really stops and never sends queued messages: queued text returns to the composer. The stopped answer stays in the transcript marked stopped and keeps its Resume until the physician sends something else.
- **News mid-answer.** Records or messages that arrive before the answer finishes are taken in.
- **News after the answer.** A finished answer stays as it was said and is marked with Update when newer records arrive.
- **Saving.** Drafts save quietly. Only a draft that can't be kept says "Not saved" with Retry, and a failed send keeps its text in the composer.
- **Lost receipt.** A command changes its item only once accepted. A lost receipt shows as pending, Coda confirms it on its own and never sends twice, and if it still can't confirm, the item says so with Check again.
- **Restart.** After a restart the physician finds conversations, drafts, history and feedback exactly as they left them, and interrupted work recovers as the [harness](harness.md#guarantees) promises.
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

A patient's message in Spruce reaches the right patient, Coda proposes a reply when one would help, and the physician revises it and sends it themselves in Spruce. Coda reads the channel and never sends patient mail in the MVP. The full behavior and its rules live in [message matching](../verification/physician/message-matching/README.md) and [message reply](../verification/physician/message-reply/README.md).

Beyond it:

- **Urgent message.** Coda flags an unlinked message that looks urgent, and its row leads Today as Now for everyone who sees it. For a linked one, Coda raises a Now concern on its row, and the draft tells the patient to call 911 or seek emergency care, for the physician to send.
- **Attachments.** Readable attachments are used, and unreadable ones are named.
- **Ask Coda to reply.** If no reply is ready, the physician asks for one and Coda always drafts one. The ask waits behind a review that's already running, and a reply from that review answers it.
- **A connected patient sends For my doctor.** It arrives through [Connected care](connected-care.md#c16-sending-for-my-doctor) like any record from the patient's Duet, marked by the patient. The practice's agents can give it more priority and judge whether a physician needs to see it. If one does, it reaches Today. When the agents pick it up, the patient's Duet shows the send as received. Nothing else goes back into the patient's Duet in the MVP.
- **After the MVP.** The card offers Send beside Copy, and Coda sends exactly the words the physician approved, shown as sent by Coda for the physician, with accepted, sent and delivered kept distinct.

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

This is how we prove principle 4, Design for the patient who does nothing.

1. A multi-week world of synthetic patients and a practice runs on the real coda-rust services. Time advances, the app closes and services restart.
2. Coda checks each patient on its schedule, reviews when something changed, and runs follow-ups on their own schedules. On staging the simulation drives this.
3. Patients never act. Coda notices what changed and carries each episode to the physician's decision and follow-through.
4. The physician sees only what needs them, and everything else is logged in Updates.
5. An undecided medication recommendation never becomes a plan, and a corrected lab retires a stale reply.
6. The run reports physician-touched items per 100 patients per week, read alongside whether necessary care happened.
7. It runs on staging. A clinician or engineer drives it from the repository with their own agent, uses the physician app, and sends an event again to rerun.
8. Simulations run the way the [harness](harness.md#how-it-works) describes. Evals are the jobs the product must do, run as simulations across patient types and scenarios.
9. The synthetic panel has a realistic spread of time since each patient was last seen. Each chart stops at the patient's last contact and ends at the simulation's today, with pending items and booked visits but nothing later.

### A1. Sign-in and access

Physicians sign in only through Chords, with an emailed link and no password or code. Accounts are invite-only. The session is shared across the browser's tabs, signing out signs out every tab, and another physician's link signs the current physician out first. The full sign-in behavior, with its model, lives in [sign-in](../verification/physician/sign-in/README.md).

Inside the workspace:

1. Which patients a physician may access, and whether a practice has roles, are not decided yet. Everything here follows that decision.
2. When a physician loses access to a patient, that patient's screens close at once and anything that physician personally started stops; practice-owned work continues.
3. Every list across patients reads within the physician's access. A list that couldn't check a patient skips only that patient and says it may be missing someone.
4. Spruce conversations show only for patients the physician may access.
5. Any practice member can invite a patient from that patient's page. To discharge a patient, the practice asks Coda. The connection itself belongs to Connected care.
6. Authorized operators inspect and adjust cadence by practice or patient. Physicians don't change it, but they can ask Coda for specific scheduled jobs for a patient.
7. Authorized Coda staff may inspect submitted feedback and the minimum evidence linked to it.
8. Drafts and seen state live on the server, not in the browser.

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
- In the MVP Coda never sends patient mail, and sends nothing into a patient's Duet except the received status of a send. It never orders and never writes back to the EHR.
- Coda never sends anything to a patient without the physician, even in an emergency. The one exception is a Connected care invitation, which a practice member starts.
- A retrieved record that tells Coda to do something is evidence, never an instruction.
- A follow-up grants no access and no authority to message, order, schedule or write back.
- Silence, elapsed time or a finished run never proves physician approval or clinical resolution.
- Code never ranks clinical importance.

Access and privacy:

- Every list reads within the physician's access, and opening a source needs access to the patient.

Data and consent:

- Synthetic data only, until we decide to work with real patients.
- Real patient messages are reviewed only through model APIs covered by a BAA.
- Coda's agents may send patient-specific questions to outside clinical AI vendors, such as UpToDate's Expert AI, under a BAA. Coda's own library stays the default.
- Each record keeps its source, date and identity and is attached to the right patient.
- A record missing from a later pull is noticed and never deleted on absence alone.
- Calls are kept only as a record that a call happened; recordings are never fetched or kept. Spruce texts, voicemail transcripts and fax pages are evidence.
- A message moved off the wrong patient is marked entered in error and never deleted. None of its content stays in that patient's record anywhere, and the flags, summaries, citations and follow-ups it produced there are retracted or re-reviewed.
- An adapter copies. It never rewrites a value and never makes a clinical call.

Evidence and honesty:

- Citations refer only to evidence the model that wrote the answer actually received.
- Any answer, reply or action about current medications or values re-reads the chart first. Memory never stands in for live chart values.
- Nothing is silently skipped. What Coda didn't read, it says, and a citation into a scan or fax points to the page.
- The library's freshness is reported as it is.
- Patient summary dates say when Coda wrote, reviewed and pulled. None of them says the patient is stable.
- No "saved" label ever shows, only "Not saved".
- A new citation requires the exact supporting passage to have reached the answering model in its current window.

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
- Working notes are separate from patient memory.

## Where Coda's information comes from

Everything Coda reads in the MVP is read only.

- **The practice's EHR.** No EHR is chosen in advance. Choose sources based on the first practice's needs, available access, and what the adapters can read.
- **For practices using Hint**, the planned adapter uses Coda's Chrome extension in the practice's own Chrome. The physician signs into Hint as usual, and the extension reads the chart's structured data in the background under that sign-in. It downloads nothing, never sees the physician's password or MFA codes, and keeps Hint's session only inside Chrome. Coda's access lasts as long as the physician's Hint sign-in. When Hint signs out, Coda keeps what it read and catches up the next time the physician signs in. Pulls run during the practice's business hours, and Coda asks the physician to reconnect only when fresh Hint data is needed and access is missing.
- **The schedule**, from the practice's EHR or another scheduler. Which one is not decided yet.
- **Patient mail** from the practice’s Spruce connection.
- **Attachments**, including scans and faxes read through OCR.
- **A connected patient's Duet data**, which lives in Coda's record for that patient and never in the EHR. Connected care owns what crosses.
- **The clinical library**, Coda's own, searched semantically and served from the cloud, always on before launch. Coda searches it as often as a question needs, reads as many sources as it judges useful, and keeps going until it has enough to answer.

Every EHR names and shapes its records in its own way. If each part of Coda had to learn each EHR, every new practice would mean changes everywhere. So Coda files every patient record into one fixed set of buckets, and search, reviews, visit preparation and citations find records through the buckets. A new source then needs only its own adapter. The adapter says where each record goes and never changes what the record says, so nothing clinical is lost or guessed on the way in.

Each adapter files every record it reads into one bucket. The bucket comes from the source's own name for that kind of record. A record whose kind the adapter doesn't know goes to Other and is never dropped. One record has one bucket, and Coda never splits a record to file its parts. The bucket only says where to find a record. Its status, dates, codes and values stay exactly as the source sent them, and what they mean is the model's judgment. The buckets are a filing index, not a schema. They limit nothing the model may read or conclude.

| Bucket | What belongs in it |
|---|---|
| Patient | Who the patient is: name, birth date, sex, identifiers and contact details. |
| Problems | Diagnoses and health concerns on the problem list. |
| Medications | Medications prescribed, dispensed or reported, including stopped ones. |
| Allergies | Allergies and intolerances, including a record that none are known. |
| Results | Results of tests: laboratory results, and imaging, pathology and other diagnostic reports. |
| Vitals | Vital signs and body measurements. |
| Notes | Notes written into the record: visit notes, phone notes, other chart notes and notes a patient saved. |
| Documents | Files kept in the chart: scans, faxes, letters and outside records. |
| Visits | Appointments and encounters. |
| Procedures | Procedures and surgeries that were done. |
| Immunizations | Vaccines given or reported. |
| Orders | Orders and referrals. |
| History | Family, social, past medical and past surgical history. |
| Forms | Forms and questionnaires that were filled in. |
| Messages | Messages between the patient and the practice. |
| Other | Any kind of record that has no bucket here. |

A record is filed by what its source calls it, never by what it contains. Where a source labels a record with a category, the category is part of the source's name for it. A lab result that arrives as a scanned file is a document. A new bucket is added here first, and adapters follow. A connected patient's Duet data comes through its own adapter. Each item is filed by what Duet calls it and keeps its origin.

Every source shows when its records were last refreshed, and one source's failure never blocks the others.

Coda's Chrome extension acts only as the signed-in physician, never takes the physician's tab or focus, and never enters credentials.

## Scope

**Now.** What exists or is in flight:

- One practice using Chords on the web, with patients on Duet, the two connected through Connected care.
- The first practice's EHR, read only.
- Spruce for practice patient messaging, with Copy reply and Open in Spruce.
- Reading attachments, including scans and faxes through OCR.
- Dictation into the composer.
- The clinical library with semantic retrieval: a documented, tag-defined PMC set that includes reviews and case reports.
- A Filtered list of mail the linking pass judged not to be from a patient, with "This is a patient" to restore it, and the same through plain words in chat.

How it is built today:

**Schedules and cadence.** Proactive review of every source (the EHR, a connected patient's Duet data) runs on scheduled jobs that pull what's new. Patient mail is handled as it arrives, and a patient joining through Connected care starts the practice's agents right away; nothing else wakes on arrival. Each connection fetches changes without model calls, and a model review runs only when something new arrived or a follow-up came due. A records check is a full re-pull of each patient compared with what Coda holds, starting daily. There's no limit on background runs in the MVP. Care-team cadence in Chords is separate from patient automations in Duet.

**Memory.** The model maintains the operating index. It is not a rigid schema, and its changes are auditable. Any change to what is kept or shared is a product decision.

**Clinical library.** The corpus pipeline is critical infrastructure: a bad article is set aside while the rest publish, a failed night resumes on its own, and the same bug twice or three failed nights in a row stops automatic attempts and tells someone why. Freshness and speed are targets, measured and reported: retractions and corrections within a day, content updates within three days, currentness re-confirmed within a week, and whole search in about 3 seconds at p95 and 5 at p99. The nightly's rules are in [Clinical library nightly](../verification/clinical-library/nightly/README.md).

**Runs.** Every job runs on the [harness](harness.md), which owns how runs start, recover, manage context and end. What's specific to Chords:

- A failed preparation, review or initial understanding retries on its own before it counts as can't restart.
- Recovery never alerts the physician.
- A refused review is recorded as a refusal with its reason, and Couldn't finish, Try again and the automatic retries read from it.
- Each patient's work starts independently and runs side by side.
  - An admission limit and priority classes come only once load measurements justify them.

**Commands and state.** Commands are accepted or not. A lost receipt resolves without a second effect, and an ambiguous command retries only itself before reading authoritative state. Reload, reconnect and another client converge on the same server state.

**Live status.** Run steps are the same on CLI and API, and the run record stays inspectable by the team. Reasoning summaries stay in the team inspector.

**Mail.** No message from a practice’s Spruce connection is filed twice or lost. After a rollback, a connection a newer release has touched stops filing and shows on the health page until an operator acts.

**Reliability.** If a startup piece can't start, the server still starts, the piece retries, and readiness reports it truthfully. Once a practice holds real data, storage changes never refuse to start or wipe data, and permanent records are never rewritten to add fields. Until then the cutover is clean, as the [harness](harness.md) says. Chart answers work without the literature service.

**Sources.** Each source has its own adapter that translates its format into Coda's common evidence, and new sources come as new adapters. An adapter's mapping is a declared table. For each kind of record the source has, the table names its bucket, the role of each date field, and where its status, codes and title are. Every record carries the same envelope: its bucket, the source's own name for its kind, its source, ID and version, its dates with their role and the source's field name, its status in the source's words, its codes, a title, and the exact record with its hash, readable text and link into the source. A record with no version from its source is versioned by its content. Search uses the envelope, and citations point into the exact record. Sources that share a format share a table. Synthetic patients come in the same way, through their own adapter. Reads happen automatically for authorized patient work.

**Not in Chords.**

- A Coda inbox of its own.
- Coda sending patient mail, ordering or writing to the EHR.
- Patient-facing voice AI.
- The visit-note scribe.
- Mail from colleagues or insurers.
- Clinical trials data in the library.
- A desktop app.

**Parked.** Nobody is working on these, and we spec each one when we pick it up:

- Read-only outside records through Kno2, with Coda acting as the physician’s delegate. These records complement the practice’s chart.
- Send beside Copy, where Coda sends exactly the approved words for the physician.
- EHR write-back, starting with a physician-approved unsigned draft note entered by computer use. A write makes exactly the change the physician approved, then re-reads the chart and reports what it now shows.
- Licensed clinical content (UpToDate, DynaMed, Wiley) once a contract lands; nothing in the MVP waits on it.
- The Chrome extension also as the minimized, ambient Coda and a way for replies to reach the native composer. Until then the web app's collapsed Coda, the orb, carries the banner and Updates.
- Coda reaching out to patients from its own findings.
- Learned physician preferences, from the physician's own words and edits only, with Undo.
- Memory that is more knowledge- or graph-based.
- Practice-wide direction from Today, such as panel sweeps.
- Rich results in chat, such as lab tables and trend charts, every value a cited record value.
- Coda-chosen visualizations of vitals that stay with the patient's understanding.
- Additional practice patient-message channels, including patient email through Gmail, native EHR messaging and text outside Spruce.
- The shared patient–physician space.
- Self-serve for hundreds of practices.
- A Hint route that doesn't depend on the practice's open Chrome.
- Coda keeping its own clinical record, or building into EHRs. Today the EHR is the source of truth, read only, and Duet data a practice receives lives in Coda.
- A limit on background runs. There's none today.
- Deleting a physician conversation, beside Archive.
