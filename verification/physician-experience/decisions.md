# Physician graph decisions

Dev's dated product decisions the graph rests on. This file owns the *why*; the
graph ([physician.mmd](physician.mmd)) owns the behavior and cites these by id or
date. A later decision supersedes an earlier one only where it says so.

Source abbreviations used here and in the graph: px.md (physician experience),
CT (AI care team operating contract), RP (runtime protocol), PM
(patient-messaging.md). Source precedence: Dev in session, then
Linear, then the settled docs, then mockups.

## Standing rules

- **Charter (2026-10-23).** Coda is a broadly capable, proactive agent team.
  Skills, tools and work kinds are additive guidance, never a closed menu. The
  graph sets hard limits only on access; communication standards; data and
  consent; and owner consistency. Everything Coda judges is a "may".
- **Communication standard (2026-10-24).** Coda never sends on its own. Only the
  exact words a physician approved go out: sent by the physician, or by Coda on
  their behalf and shown as sent by Coda for them (PM:160, 992).
- **Access has one owner (2026-10-26).** The physician's access to a patient
  (clinical, identity only, or none) is one region every patient command
  reads: acting on a patient or their linked message needs at least identity
  access, and anything showing or changing clinical content needs clinical
  access. Roles inside the practice are practice policy, not graph state; only
  the messaging manager is modeled, as the messaging contract names it.
- **A concern always comes with its explanation (2026-10-26, from the care-team
  contract's visibility table).** Coda raises or revises a concern only in work
  that writes the physician a communication explaining it, and the concern
  opens that work. A quiet finish may only close concerns. The link from each
  concern revision to its communication is a data promise for a linked proof,
  not graph state.
- **Behavior, not presentation (2026-10-26).** Layout, open and close, hover
  and paging belong to the surface map and the app, not the graph, as in
  Duet's. The graph keeps what the physician can do, what Coda does, and what
  is guaranteed. It keeps what is open only when a read, an access limit or a
  guarantee depends on it (the reader, Updates, Find patient, a source pane
  and its load); how things open is the app's. A guarantee that lived in a
  screen detail stays as a plain rule:
  - Opening an update is one event wherever the app offers it; previewing one
    never navigates or clears attention, only opening it does. Collapsing Coda
    to the orb is presentation.
  - The patient page shows what the summary rests on and its limits (Basis)
    and the sources Coda read (Sources); a Sources row opens that source's
    citation. Opening and closing them is presentation.
  - Sidebar and schedule collapse are remembered per physician (M8); they are
    preferences of the surface, not graph state.
  - Running work may show its current step, only from a real tool call
   ; the step line is the surface's. The graph keeps that the work
    is running.
  - Find patient searches the whole roster on the server, and every roster
    patient stays reachable; Updates keeps every update of the day reachable.
    How results and the day page in is the surface's.
  - A source pane belongs to the surface that opened it and closes with it;
    how many show at once is the surface's.
  - A draft is kept quietly; how it saves is the app's. When it can't be kept,
    it says Not saved with Retry, and nothing typed is lost (N3).
- **Ideal state, not infrastructure (2026-10-24).** The graph is the ideal
  behavior of the full product for any user: what they see and how things act and
  work for them. It leaves out runtime capacity, retries, timeouts, interruptions,
  sync and milestone stopgaps; [infrastructure-gaps.md](infrastructure-gaps.md)
  lists what the real system must still guarantee. Failures caused by the outside
  world stay: a source that is down, a channel that refuses, evidence that can't
  be read. This supersedes the 2026-10-23 principle "retries everywhere": the
  graph shows outcomes, and how often the runtime retries is not the graph's.
- **Today and Updates (2026-10-23).** Today is only what needs the physician;
  Updates is the day's record of everything Coda did, which the physician never
  maintains.

## 2026-10-22

- **Done is shared.** Done on a concern clears that exact concern for every
  physician; another physician's Done clears it for you too.
- **One owner for the patient's connection.** The practice side keeps one
  connection lifecycle, the invitation included. The roster shows its status
  without notifications. An ended connection can be invited again, and consent
  wording is fixed for the MVP, so there is no re-consent state (with the
  2026-10-23 decisions below).

## Tie-out decisions (2026-10-23)

1. Work that ends with nothing left to do adds no Today row; Updates logs it with
   its reason.
2. When Coda can't restart work, it goes to Today with what was left undone and
   Done only if something is left unhandled; otherwise Updates only.
3. A patient message Coda can't read puts a Today row on the patient, linking to
   it in Gmail or Spruce. It leaves when Coda reads it or the physician marks it
   Done.
4. When Today partly fails to load, it shows what loaded plus one quiet line with
   Retry. The same holds for Updates and Find patient.
5. A concern, briefing, follow-up or past work that fails to open keeps the reader
   on it with "Couldn't load this" and Retry. Older content stays visible.
6. Can't-restart work that waits on Today with Done: patient review, follow-up
   review, visit preparation (leaves when the visit ends) and asked reply. Initial
   understanding and weekly review end straight into Updates.
7. When records change under a chat answer or an asked reply, Coda finishes from
   what it read and offers one tap to update. Changes from that answer wait for
   the update. *Revised 2026-10-24:* any change in records counts (not only a
   second one), and understanding, review and visit preparation look again on
   their own before finishing. *Revised 2026-10-26:* whatever Coda is working
   on takes in what arrives meanwhile before it finishes, so nothing it
   publishes rests on an older reading; the patient review still owns
   responding to news. A chat answer already given is what was said: it keeps
   the records it used, marked when newer ones arrive, with Update. This
   replaces finishing on older records and RP:1430-1432's publishing a
   reassessment as it is.
8. One receipt rule for every physician command: a command changes its item only
   when accepted; a lost receipt shows as pending and never sends twice; only
   if it still can't be confirmed does the item say so, with Check again.
9. When a records source is down, each patient page and briefing says
   "Records as of …, couldn't update". A source that comes back keeps its last
   good "Records as of …" until it actually updates. When the practice's whole source is down,
   Today carries one line for the practice, never a row per patient. Coda keeps
   working from the last good records. *The practice-wide line was approved for
   the graph 2026-10-24.*
10. When a review proposes a reply to a newer message while "No reply needed"
    shows Undo, the new reply wins and the old Undo ends quietly.
11. Chat follows queue-versus-steer: Send while Coda is answering steers the
    current answer; Send next queues, editable. Typed input is never lost.
    *Revised 2026-10-24:* a steer applies at once; there is no mid-recovery wait.
12. A new message supersedes a stopped answer. The stopped answer stays in the
    transcript marked stopped, and Resume exists only until the physician sends
    something else.
13. Coverage: reader navigation, every update of the day and earlier days, the orb, Find
    patient's read and failure, unavailable citations, and thumbs on briefings and
    reviews.
14. Session expiry is not sign-out. A sign-in prompt covers the screen and hides
    patient content until the physician signs in again; afterwards they are exactly
    where they were, and running work is unaffected.

Applied with these, from sources: a calendar lost mid-session keeps the last good
day (PX-147); deleting the patient's account while invited cancels the invitation;
a match decision or its Undo wakes the review (PM-331); physician chat
can wake another initial-understanding attempt (CT-1196).

Also 2026-10-23: any signed-in physician can match a patient message; the
patient's connection means sharing, with no pause; a stopped follow-up is
held until Resume (supersedes CT:98's "Pause is removed entirely").

## Node-and-edge audit decisions (2026-10-23)

- **A1.** Stop pauses the queue: nothing is lost or sent. Resume continues the
  stopped answer, then the queue sends. Queued messages stay visible, editable and
  deletable. *Revised 2026-10-24:* a queued message can be sent at once while no
  answer is in progress.
- **A2.** Send carries the draft's text itself; autosave is only a safety net. A
  failed send keeps the text in the composer under decision 8.
- **A3.** Correcting a confirmed match (Wrong patient?) stays with the messaging
  manager, going forward only (Dev 2026-10-01; px.md:169; PM:543-545). Matching
  itself is open to any signed-in physician.
- **A4.** Done on "Coda can't restart the follow-up review" offers the
  follow-up's own choices: complete, set a new date, or cancel. The follow-up
  always keeps a next step (CT:118).
- **A5.** Quiet work may create, revise or complete follow-ups, and every change
  is logged in Updates. After a physician cancels one, Coda doesn't recreate it
  unless new information arrives.
- **A6.** An unmatched message that looks urgent may be marked so by the linking
  pass; its Messages-to-match row then leads Today as Now for everyone who sees
  it. Supersedes PM:401 for unmatched rows.
- **A7.** Any work Coda judges may tie a concern to an upcoming visit; it ends
  with the visit. A concern about a patient's message stays with that message.
  Supersedes RP:233-235's preparation-only rule.

## 2026-10-24

- **Ask Coda to reply waits** behind a patient review already running; a reply
  that review proposes first answers the ask (px.md:167).
- **Send beside Copy.** The reply card offers Send, which has Coda send exactly
  the approved words, shown as sent by Coda for the physician. This is the ideal
  state; direct sending is not in the MVP, where the physician copies the reply
  and sends it in Gmail or Spruce (Dev, 2026-10-24).
- **Work runs side by side.** There is no per-patient work slot; only real
  dependencies order work, such as later work building on the initial
  understanding.
- **No Interrupted state.** Work that can't finish shows Couldn't finish with Try
  again; recovery is the runtime's.
- **Steer applies at once** (revises decision 11).
- **Update restores** an archived conversation, like Send, Resume and Try again.
- **One conversation read carries its draft**; there is no separate draft wait.
- **Matching ends with a person** whenever Coda can't tell; how long the runtime
  tries is not the graph's.
- **Channel sync is the channel's.** Catching up and temporary sync failures are
  never shown; only a revoked grant or an administrator's block needs Reconnect.
- **Outreach is parked.** Not in the graph until Dev raises it.
- **Sign-in by email link, connected to the graph** (answers while connecting
  sign-in.mmd from #89):
  - Asking for a link never reveals whether an account exists; an address without
    one is sent nothing. Opening a link that no longer grants access to this
    practice shows "No access to this practice".
  - After the session expires, the prompt emails a link to the same address.
    Opening it resumes the covered page exactly where the physician was; the
    link's own tab opens Today, since the email carries no place (below). If
    Coda was closed meanwhile, reopening shows the sign-in page and signing in
    opens Today.
  - While signed in, the physician's own link changes nothing; another
    physician's link ends this session and opens Today for them, with drafts and
    seen state kept per physician (N7).
  - **The sign-in email carries no patient** (Dev, "keep it simple"). A Coda
    link opened while signed out is held by the tab that opened it. Signing in
    in any tab of the browser signs in its signed-out tabs, as sign-out reaches
    every tab, so the holding tab lands on its patient; the emailed link's own
    tab opens Today.
- **Records refresh overdue is left out.** A scheduled pull that didn't happen,
  while the source is up, is runtime scheduling; keeping records current is on
  the infrastructure-gaps list, and a source that is actually down is decision 9.

## 2026-10-25

- **Connected to Duet through shared events.** The practice connection meets
  Duet's in [connected-care](../connected-care/connection.mmd): each graph owns
  its side, and rules there hold both sides to one status.
- **Any open invitation can be accepted** (Dev, 2026-10-25). A patient who taps
  Share on the invitation they hold connects, even while the practice resends it
  or a resend couldn't be delivered; both sides show Connected.
- **Joining shares what the patient already has** (Dev, 2026-10-25). The join
  is news for the patient: their connected records arrive with it, so Coda's
  work is owed at once, on a first join and on joining again after the
  connection ended. The patient's For my doctor arrives only when they send it,
  like a message from them (Dev, 2026-10-26).

## Audit findings

The graph, the standing rules and infrastructure-gaps.md cite these findings
from the 2026-10-23 audits, each of which settled a behavior from the sources:

| Id | Behavior | Source |
| --- | --- | --- |
| N1 | Every work kind runs on any authorized source, Duet included. | CT:1203-1205 |
| N2 | From a correction onward, items built on the moved message are withheld until the old patient's re-review revises, re-authors, ends or closes each. | PM:640-711; Dev 2026-10-03 |
| N3 | Typed input is never lost, including when access is lost or the session ends. | Linear product requirement |
| N5, N9 | Dictation stores no audio and has no audio retry; expiry cancels a recording and keeps the text. | px.md:349-350 |
| N7 | A different physician starts at their own Today; drafts and seen state are per physician. | px.md |
| N8 | Resume re-reads from an authorized source pull. | px.md:130 |
| N10 | Closing a reply as answered doesn't change an edited reply box. | PM |
| N11 | A failed records check doesn't wait a day. | CT; decision 9 |
| N13 | A message during initial understanding gets its own review afterwards. | RP:810-811 |
| M1 | A stopped follow-up can still be changed; Resume reviews the current version. | CT |
| M8 | Sidebar and schedule collapse preferences are remembered per physician. | px.md:617 |
| X1 | A completed follow-up never has its review running. | CT:157 |
| X4 | Asking Coda to reply needs a connected channel. | PM |
| bug 8 | Urgency belongs to the concern and exists only while it needs attention. | audit |
| bug 15 | Physician commands need an active session. | audit |
