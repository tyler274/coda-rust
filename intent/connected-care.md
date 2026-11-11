# Connected care

This is how a Duet patient and a Coda practice connect: what crosses between them, who controls it, and what happens when the connection ends. It describes the product as we intend it today. Git keeps the history.

## What Connected care is

Connected care joins two separate products. Duet is the patient's own health app, and Chords is the practice's team of agents. A practice invites a patient, the patient joins and consents once, and from then on what the patient connected in Duet reaches the practice live. In the other direction, the practice's documented care decisions and appointment details come back to the patient's Duet.

The MVP includes practice invitations, consented records and readings, and what the patient sends from Shared. Patients can connect personal sources such as Apple Health on their own. In the MVP, the only thing that comes back from the practice into Duet is the status of a send. The doctor's plan, practice messages, Check-ins and appointment preparation come later. Their shape is not decided yet, and Scope lists them as parked.

The point is that the data a patient can connect actually gets to their physician. The physician sees what the chart doesn't hold, such as the patient's other health systems, phone health readings and the notes they've made, and the patient understands what their clinician documented, without anyone running an approval queue for Duet. Connecting patients to their providers is where we ultimately add the most value, and it has to reduce work for both sides rather than create more.

## Principles

Coda's shared principles (README.md) govern this spec, and what either side shows of the connection follows the shared UI principles (U1–U9). These are the ones specific to Connected care.

1. **Two products, two owners.** Each product keeps its own data and identity, and neither reads the other's storage. The patient account belongs to Duet; Connected care attaches connections to it and owns no accounts. Physicians sign in only through Chords.
2. **Duet is the patient's; the chart is the practice's.** What a practice receives becomes part of its record, held in Coda's record for that patient and never written to the EHR. Duet stays the patient's after any ending.
3. **A connection is permission for data to flow, and nothing more.** The patient's shared data goes to that practice, and the practice's documented care decisions and appointment details come back.
4. **Connected means sharing, and sharing is live.** While connected, what the patient connected in Duet reaches the practice continuously. There's no paused state; the patient stops sharing by leaving. Joining, the first time or again, shares what the patient already has.
5. **The patient alone decides whether sharing happens.** What crosses is what they connected in Duet. One consent record per connection governs everything that crosses, only the patient can change it, and a practice discharge also ends it. Saving something in Duet shares nothing by itself.
6. **Clinical authority comes from documented care decisions.** Physicians don't approve Duet's wording, approve a separate patient plan or manage the patient's Duet, and Duet never claims a physician reviewed anything. Documented decisions reaching Duet are updates, not messages, so they need no approval. Any message from the practice to the patient does need the physician's approval.
7. **Faithful, never invented.** Duet explains the doctor's plan faithfully, keeps a documented decision, an unconfirmed suggestion and a patient report distinct, and never invents or amends care instructions. Ambiguity in the source stays visible.
8. **Everything keeps its origin and date.** Every item that crosses carries where it came from and when.
9. **One status, both sides.** Duet and the practice always show the same connection status.
10. **Less work for both.** The practice's agents read what arrives and judge whether a physician needs to see it. Check-in answers never become alerts, and the roster shows status without notifications.
11. **Ending preserves.** Ending stops future sharing but keeps the patient's account and history, the practice's dated copies, and the last doctor's plan, marked as no longer updated. Anything already said or sent keeps the version it used.
12. **Duet stands alone.** Duet is useful without a practice, and a connection adds to it. The patient's own sources and appointment context stay separate from what a practice provides, and practice-supplied facts keep their practice as the source.
13. **Works for the patient who does nothing, and is honest about their data.** Nothing in the connection depends on the patient doing work, and they always know what happens to what they share.
14. **Nothing assumes a visit.** Inviting, joining and the plan coming back never depend on a visit happening.
15. **No silence that looks like health.** If sharing is behind, stuck or degraded, someone who can act sees it, with the reason.

## The concepts

Some concepts below say only what the thing is. Their rules are not decided yet, and Scope lists them as parked.

**Connection.** One patient and one practice. The roster shows three statuses: Invited, Connected and Ended. A connection ends when the patient leaves, the practice discharges them or the account is deleted. A physician leaving the practice doesn't end it; their patients are reassigned. A patient may connect to more than one practice, each connection separate, and ending one leaves the others alone.

**Practice members.** Anyone signed in to the practice can invite a roster patient from that patient's roster row. To discharge a patient, the practice asks Coda, and Coda ends the connection at once.

**Invite.** From a practice to one person on its roster. It's sent, then accepted or cancelled. It stays open until the patient joins or the practice cancels it, and practice members can resend it. The patient can accept any invite the practice still holds open, including one being resent, and then both sides show Connected.

**Consent.** One record per connection, granted for a given wording. In the MVP the wording is fixed and consent is given once, at join, by tapping Share, after a plain preview of what it shares. Only the patient changes it, and in the MVP the way to change it is to leave. Data crosses only while connected and consent is current.

**What the patient shares.** What the patient connected in Duet: portal records, phone health readings and medications they entered. A For my doctor note crosses only when the patient sends it. Duet's AI keeps private notes about the patient, and those never cross. Copies of records the practice’s chart already holds arrive labeled as duplicates, and a corrected record arrives as a new version.

**How the practice picks it up.** A patient joining or rejoining starts the practice's agents on what they shared right away. After that, the practice's agents pick up what arrives on their scheduled reviews and judge what matters to the physician. This is the practice's care-team work, separate from Duet's own work for the person.

**For my doctor.** The records the patient marked and the notes they wrote for their doctor. It belongs to Duet and is separate from the connection. Connected care owns only the sending: while connected, the patient sends marked records and notes to the practice. A send arrives once, as sent, like any shared record, marked by the patient. Marking or writing shares nothing, and nothing in it is streamed live. Connected records already reach the practice live, so a send grants nothing new. The mark tells the practice's agents what the patient wants them to notice. The agents can give it more priority. The practice returns one status to Duet: received, once its agents have picked the send up. It is the first step toward the shared space under Parked.

**Your doctor's plan.** Documented care decisions and open items from the connected practice, each with its source, author, date and status, read-only in Duet. Only what is final in the EHR goes, such as signed notes, active medications and orders, and the problem list. Unsigned drafts never go.

**Appointment details.** The practice's appointment facts, sent back over the connection. They keep their schedule as the source and never authorize new clinical instructions.

**Plan Check-in.** A Duet Check-in tied to the doctor's plan.

**Roster status.** The practice's only view of the connection itself: Invited, Connected or Ended on the roster, with no notifications.

## Journeys

### C1.1 Invitation

1. A practice member invites a patient from that patient's roster row.
2. Coda sends the invite by email.
3. The practice sees Invited on the roster, with no notifications.
4. The invite stays open until the patient joins or the practice cancels it.
5. Practice members can resend or cancel the invite while it's open.
6. If the patient taps Share on an invite the practice still holds open, they connect and both sides show Connected, even while a resend is under way or couldn't be delivered.
7. A patient already connected to another practice can join this one too. Each connection is separate, and the existing one is unaffected.

### C1.2 Joining with consent

1. The patient taps the invite link. It opens Duet if it's installed; otherwise it goes to the App Store or Play Store and the invite picks up after install. A short code works as a fallback. Duet has no web surface, so every invite opens the app.
2. The patient signs in, or signs up if they're new to Duet. Invites use email sign-in, and joining from an invite links to the account they already have.
3. The patient confirms their date of birth. Wrong guesses lock the code.
4. The patient sees the practice, their physician and a plain preview of what the connection shares.
5. The patient taps Share. That one act grants consent for the fixed wording and connects.
6. Joining shares what the patient already has. Their connected records reach the practice with the join, with no further tap.
7. The practice's agents start on those records right away.
8. The roster shows Connected, and Duet shows the same.
9. A patient new to Duet can connect records and use Shared without starting a general AI conversation.

### C1.3 Consented records reaching the physician

1. While connected, what the patient connected in Duet reaches the practice continuously: portal records, phone readings and medications they entered.
2. Duet's AI's private notes about the patient never cross.
3. The practice's agents pick up what arrived on their scheduled reviews and judge whether a physician needs to see it.
4. The physician sees Duet data in the patient's chart, reviews, preparation and answers, each item labeled with its origin and date ("from Epic, via the patient's Duet"), and reads each record exactly as it was sent.
5. Copies of records the chart already holds arrive labeled as duplicates.
6. When a record is corrected at its source, the new version arrives as its own event. Unfinished work doesn't publish from the version it replaced, and the practice keeps the earlier version it received, dated.
7. The patient sees no "your care team looked" receipt on anything they share.
8. If sharing is behind or failing, someone who can act sees it, with the reason. The patient sees problems with their own sources in Duet, and Coda's operations team sees problems in the sharing pipeline.

### C1.6 Sending For my doctor

1. The patient marks records and writes notes in For my doctor. Neither shares anything.
2. While connected, the patient taps Send. The button names the practice it goes to. Without a live connection there's no Send.
3. Each record in it goes at its current version. A record corrected at its source says it was corrected and when, and sending is never blocked.
4. It reaches the practice once, as sent, linked to the patient like any shared record and marked by the patient.
5. Duet hands what the patient sent to the practice's care team through the connection. Duet does not need a patient-side conversational agent for this.
6. The practice's agents read it. They can give it more priority because the patient marked it. They judge whether a physician needs to see it.
7. When the practice's agents have picked the send up, Duet shows it as Received by the care team. Nothing else comes back into Duet from a send in the MVP.
8. The status never says that a physician looked, and Duet never claims a physician reviewed it.
9. A send is fixed. Later marks, notes and edits never change it.
10. Joining a practice sends nothing from For my doctor.

### X1. Patient leaves

1. The patient leaves the practice in Duet.
2. The connection ends everywhere at once, and new data stops.
3. The patient keeps their Duet account and history.
4. Nothing from the practice updates after the end.
5. The practice keeps the dated copies it already received, and the roster shows Ended, with no notification.
6. In the MVP, leaving is the only way to stop sharing, short of deleting the account.

### X2. Practice discharges the patient

1. The practice asks Coda to discharge the patient, and Coda ends the connection.
2. The connection and the data flow end at once.
3. Duet shows the connection as ended, with the date.
4. The practice's own notice obligations never extend Duet's sharing permission.
5. The patient keeps their account and history.
6. The practice keeps the dated copies it already received, and the roster and Duet both show Ended.

### X3. Account deletion

1. The patient deletes their Duet account.
2. Every connection ends and sharing stops at once.
3. Each practice keeps what it already received, dated.
4. The practice roster shows Ended.

### X4. Physician leaves the practice

1. The physician's panel is reassigned, and the connection continues.
2. Telling the patient who their doctor is now comes later.

### R1. Re-invitation and rejoining

1. After a connection ends, whether the patient left or was discharged, any practice member can send a new invite at any time. For inviting, Ended is the same as never connected.
2. The patient accepts and consents again, through C1.1 and C1.2.
3. Joining again shares everything the patient has, including what they added while disconnected, and the practice's agents start on it right away.

### What each side sees

| Step | The patient in Duet | The practice |
|---|---|---|
| Invite sent | An email with a link or short code | Invited on the roster, no notifications |
| Open invite | Stays open until they join | The status, with resend or cancel |
| Join | Date of birth check, the practice and physician, a plain preview, Share | Connected; existing records arrive and the agents start |
| Live sharing | No activity view and no "care team looked" receipt | Items labeled with origin and date in the chart, reviews, preparation and answers, duplicates labeled |
| For my doctor | Send, records at their current version, a Sent and Received status | Arrives once, like any shared record, marked by the patient. The agents judge whether a physician needs to see it |
| Leave | Account and history kept | Ended, dated copies kept |
| Discharge | Ended, with the date; account and history kept | Ended, dated copies kept |
| Account deletion | Account gone | Ended, copies kept |
| Physician leaves | Later: who their doctor is now | Panel reassigned |
| Rejoin | New invite and consent | Connected; existing data arrives again |

## Rules

1. Nothing crosses without a current connection and current consent.
2. One consent record per connection decides everything that crosses, and only the patient changes it.
3. Connected means sharing: there's no paused state, and the roster shows only Invited, Connected or Ended.
4. Leaving, discharge and account deletion end the connection everywhere and stop new data at once.
5. Duet and the practice always show the same connection status.
6. Any practice member may invite or re-invite. Coda discharges a patient when the practice asks, and it takes effect at once.
7. The practice keeps dated copies of what it already received after any ending.
8. Ending keeps the patient's Duet account and history, and nothing updates after the end.
9. On discharge, data stops at once and Duet shows the connection as ended, with the date. The practice's notice obligations don't extend sharing.
10. Duet's AI's private notes never cross.
11. For my doctor crosses only when the patient sends it, once, as sent, with each record at its current version, and a corrected record says so.
12. Every item that crosses keeps its origin and date.
13. A corrected record is a new version and its own event, and unfinished work never publishes from the version it replaced.
14. Records the chart already holds arrive labeled as duplicates.
15. Physicians don't approve Duet's wording or manage Duet, and Duet never claims physician review.
16. Every message from the practice to the patient needs the physician's approval.
17. Duet never invents or amends care instructions, and an undecided recommendation stays an unconfirmed suggestion.
18. Appointment details keep their schedule as the source and never authorize new clinical instructions.
19. Plan Check-in answers never become alerts.
20. There's no "your care team looked" receipt on anything the patient shares.
21. Each request sees only its own person's data.
22. Adults 18 and over only.
23. Connection status has one owner that both sides read.
24. Neither product reads the other's storage. Data crosses only through the connection.

## Scope

**Now.** What exists or is in flight:

- Practice connections, each started by an invite, with fixed consent at join, sharing until the patient leaves, the practice discharges or the account is deleted.
- Connections to more than one practice, each separate.
- Live sharing of connected records and readings.
- Sends from Shared, with the records the patient marked and the notes they wrote.
- The practice's agents review each send like any shared record, marked by the patient. Duet shows the send as received once they have picked it up.
- Ending a connection with the account and history kept, and re-inviting after an ending.
- The physician on Chords on the web; patients on Duet on iPhone and Android, with no Duet web surface.

How it is built today:

**Shared events.** Both products see the same events: an invite sent, accepted or cancelled, a join, a leave, a discharge, an account deletion, the live feed of shared items, and a For my doctor send with its received status.

**Endings.** An ending takes effect at once, and no item crosses after it.

**Delivery.** A For my doctor send is delivered exactly once, as sent. A record correction is its own event and reaches unfinished work before that work publishes. A join delivers the patient's existing records and starts the practice's agents.

**Schedules.** Apart from a join or rejoin, the practice's review of Duet data runs on scheduled jobs that pull what's current.

**Failures.** Failed or slow pulls retry, then degrade honestly with the date of what's shown, and repeated failure becomes visible to someone who can act.

**Not in Connected care.**

- Anything from the practice into Duet beyond the status of a send, including answers to the patient.
- Pausing and resuming sharing, withdrawing consent, and consenting again after a wording change.
- Patients starting a connection through clinician lookup, and saving a care team.
- Sharing with doctors outside Coda, which comes later as a browser review of selected records with a short overview and no PDF download.
- Caregivers, proxies and minors.
- Telling the patient their new doctor after a physician leaves.
- Release of information.
- Any "coming soon" entry. Features we don't support are absent from the app.

**Parked.** This is the ideal product. Nobody is working on these, and we spec each one when we pick it up:

- Invitations by text, and an automatic reminder and expiry for an invite.
- A discharge control on the roster, and a discharge notice with its wording.
- The doctor's plan in Duet, with a plain-language explanation and plan Check-ins. Each practice's plan keeps its own label, and the rules for ending a connection apply to it.
- Appointment context from the patient and the practice, and AI preparation. Ending a connection removes the practice's appointment and keeps the patient's own date and saved preparation.
- Notes from conversation and practice news in Duet's one thread, over the same connection.
- A shared space for each patient and physician, maintained together, with two-way replies and co-editing. The practice invites, as it does now, and it starts with just the two of them.
- In that space each side's agent drafts and its person confirms, and the patient's confirmation of a plan is a signal, never a gate.
- It needs the physician on Chords. Specialists who aren't on Coda get a read-only view that grows out of For my doctor.
- Urgent symptoms send the patient straight to call the practice or 911 and never wait in the space.
- Every message from the practice to the patient still needs the physician's approval, and Coda never answers the patient from standing rules.
- The patient as co-owner of the record.
- NPI registry ingestion as its own groundwork.
