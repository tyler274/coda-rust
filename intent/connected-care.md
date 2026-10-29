# Connected care: intent spec

How a Duet patient and a Coda practice connect. Draft for Dev to correct. Built from the verified evidence bundle, Dev's answers to agents' multiple-choice questions, Dev's full messages and Linear, after an adversarial review.

**Pointer key.** `linear:Connected care (V1)` is the Linear project; `linear:coda-rust platform`, `linear:Coda Agents` and `linear:Duet` are projects; `linear:ENG-n#Section` is a ticket section. `codex:<file>@<time>` and `claude:<file>@<time>` are Dev's own words at that UTC time. `(chosen)` marks the option Dev clicked in an agent's multiple-choice question; it is binding. `doc:` and `(doc)` mark older agent-written docs, which fill detail only where Dev and Linear are silent.

---

## 1. Purpose

Connected care joins two separate products: Duet, the patient's own app, and Coda Agents, the practice's physician system [linear:Coda Agents#Duet–Agents connection; codex:rollout-2026-10-14T10-42-27-01a13add-5ae0-73f3-80a2-333b09de@2026-10-14T15:59:18.524Z]. A practice invites a patient; the patient joins and consents once; from then on what the patient connected in Duet reaches the practice live, and the practice's documented care decisions and appointment details come back to Duet [linear:ENG-70#The model; linear:coda-rust platform#Journeys]. What matters most is that "the data that the patient is able to connect gets sent to the physician" [claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T17:41:22.927Z]. It lets the physician see what the chart doesn't hold and lets the patient understand what their clinician documented, without a physician approval queue for Duet [linear:ENG-39#Why it matters; linear:ENG-45#Why it matters]. "Our ultimate value will be how we connect the patient to their providers" [codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T16:58:43.155Z].

## 2. Principles

Coda's principles (PRINCIPLES.md) govern this spec. These are the Connected care-specific ones. What either side shows of the connection follows the shared UI principles (PRINCIPLES.md U1–U9).

1. **Two products, two owners.** Each product keeps its own data and identity; neither reads the other's storage. The patient account is Duet's; Connected care attaches connections to it and owns no accounts. Physicians sign in only through Agents. [linear:Connected care (V1)#Features and cadence; linear:ENG-70#The model; linear:ENG-27#Decided (Dev, 2026-10-12)]
2. **Duet is the patient's; the chart is the practice's.** What a practice receives becomes part of its record; Duet stays the patient's after any ending. [linear:ENG-70#The model; linear:ENG-70#Flows] Received Duet data lives in Coda's record for that patient and is never written to the EHR [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:12:44.566Z].
3. **A connection is only permission for data to flow.** The patient's shared data goes to that practice; documented care instructions and that practice's appointment details come back. [linear:ENG-70#The model]
4. **Connected means sharing, and sharing is live.** While connected, what the patient connected in Duet reaches the practice continuously. There is no paused state: "Connected means sharing. The patient stops by leaving (connection ends); there's no paused state and no gap." Joining, first time or again, shares what the patient already has. [linear:ENG-70#MVP scope; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen); linear:ENG-39#Decided; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T19:53:52.075Z]
5. **The patient alone decides whether sharing happens.** What crosses is what they connected in Duet. One consent record per connection, which only the patient can change, governs everything that crosses; a practice discharge also ends it. Saving alone shares nothing. [linear:ENG-70#Done when (MVP); linear:ENG-39#Decided; linear:ENG-70#Flows]
6. **Clinical authority comes from documented care decisions.** Physicians don't approve Duet's wording, approve a separate patient plan, or manage the patient's Duet. Duet's wording never claims physician review (DECISIONS.md B1). [linear:Connected care (V1)#MVP outcomes; linear:ENG-45#Decided; codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T18:18:03.151Z] This covers Duet's wording and updates such as documented decisions reaching Duet; any message from the practice to the patient still needs the physician's approval [codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T18:18:03.151Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z].
7. **Faithful, never invented.** Duet explains the doctor's plan faithfully, keeps a documented decision, an unconfirmed suggestion and a patient report distinct, and never invents or amends care instructions. Ambiguity in the source stays visible. [linear:ENG-45#Done when; linear:ENG-45#Discuss with Dev when you get here]
8. **Everything keeps its origin and date.** Every item that crosses carries where it came from and when. [linear:ENG-39#Decided; linear:ENG-39#What the patient and physician can do; doc:coda-app/north-star.md:1972]
9. **One status, both sides.** Duet and the practice always show the same connection status for a patient. [linear:ENG-70#Done when (MVP); linear:ENG-39#Don't repeat; linear:ENG-185#What the graphs cover]
10. **Less work for both, not more.** "it's meant to reduce friction and work for both parties, right? Not make more." The practice's agents read what arrives and judge whether a physician needs to see it; Check-in answers never become alerts; the roster shows status with no notifications. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T22:41:31.840Z; linear:ENG-39#Decided; linear:ENG-14#Decided; linear:ENG-70#Flows]
11. **Ending preserves; history stays as it was.** Ending stops future sharing but keeps the patient's account and history, the practice's dated copies, and the last doctor's plan, marked no longer updated. "What was already said or sent keeps the version it used, marked when newer exists; live applies to what you're looking at and working on." [linear:Connected care (V1)#Current MVP scope; linear:ENG-70#Done when (MVP); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:32:53.088Z]
12. **One fact, one owner.** Every screen and job reads the one owner; no parallel owners or forks. [linear:coda-rust platform#What went wrong before; codex:rollout-2026-09-11T15-36-57-01a091f9-1e58-7892-9c38-90e31364@2026-09-11T23:06:42.650Z]
13. **No silence that looks like health.** If sharing is behind, stuck or degraded, someone who can act sees it, with the reason. [linear:coda-rust platform#What went wrong before; codex:rollout-2026-09-09T22-56-37-01a0893e-ecdd-7212-b390-c0ef653d@2026-09-11T01:25:39.366Z]
14. **Duet stands alone.** Standalone Duet remains useful without a practice; connection adds to it. [linear:Duet#Current MVP scope; codex:rollout-2026-09-17T10-12-36-01a0afb6-513f-7f02-b349-2f830eb0@2026-09-18T00:43:14.064Z]
15. **The patient's own sources stay separate from what the practice provides.** "I think we need a patient-facing side of data sources and connection that's separate from provider-facing. I don't want you to fucking context swap the two." Duet's own records and appointment context work without a practice; practice-supplied facts keep their practice source. [codex:rollout-2026-10-20T23-29-08-01a15c81-6d66-7c00-8ba7-351f7c7e@2026-10-21T14:17:16.812Z; linear:ENG-70#The model]
16. **Works for the patient who does nothing, and is honest about their data.** "It has to work for a patient who does nothing, and be honest about what happens to their data." [linear:ENG-70#Why]
17. **Nothing assumes a visit.** Inviting, joining and the plan coming back don't depend on a visit happening: "Dude, stop assuming that things are happening in the visit." [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T20:10:44.949Z]

## 3. Mental models

**Connection.** One patient, one practice. The roster shows three statuses: Invited, Connected, Ended. It ends when the patient leaves, the practice discharges, or the account is deleted. A physician leaving the practice does not end it; their patients are reassigned. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen); linear:ENG-70#Four parts, each with its own states; linear:ENG-70#Done when (MVP)] A patient may connect to more than one practice; each connection is separate, and ending one leaves the others unaffected. Dev: "No, the MVP should not just allow one connection." [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z; linear:ENG-70#Flows]

**Practice members.** Anyone signed in to the practice can invite or discharge a roster patient from that patient's roster row; discharge asks for confirmation and takes effect immediately. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:56:50.590Z (chosen); linear:ENG-70#Flows; linear:Connected care (V1)#Current MVP scope]

**Invite.** From a practice to one person on its roster. States: sent, then accepted, expired or cancelled. Practice members send, resend or cancel it. One reminder, then it expires. The patient can accept any invitation the practice still holds open, "including one being resent or that couldn't be redelivered. The patient said yes, so both sides show Connected." [linear:ENG-70#Four parts, each with its own states; linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T17:12:54.534Z (chosen)]

**Consent.** One record per connection: none, or granted for a given wording version. In the MVP the wording is fixed and consent is given once, at join, by the Share tap, with a plain preview of everything it covers, including appointment details. Only the patient changes it, and in the MVP the only way to change it is to leave. Data crosses only while connected and consent is current. Withdrawal and re-consent are later work. [linear:ENG-70#Four parts, each with its own states; linear:ENG-70#The model; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen); linear:ENG-70#MVP scope]

**What the patient shares (the live feed).** What the patient connected in Duet: portal records, phone health readings, and notes and medications they entered. What the patient tells Duet's AI crosses only as a note the patient saved: one they asked Duet to save, or one Duet proposed and they approved. The AI's private notes never cross. Plan Check-in answers reach the practice's next Agents review. Copies of records the practice chart or Kno2 already holds arrive labelled as duplicates. A corrected record arrives as a new version, its own event. Nothing wakes on arrival: the practice's proactive review runs on scheduled (cron) jobs, each pulls what's current, and the agents judge what matters to the physician. Cost and frequency are runtime budget (Tyler's) [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z (chosen); claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:51.881Z]. This is the Agents care-team cadence, separate from Duet's own work for the person: "Don't don't confuse them" [linear:ENG-14#Discuss with Dev; claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T17:43:05.524Z]. [linear:ENG-39#What the patient and physician can do; claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T14:56:06.803Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T18:58:16.345Z (chosen); linear:ENG-39#Decided; linear:ENG-45#Done when; linear:ENG-39#Done when]

**For my doctor.** The patient's own prepared summary for their doctor. The workspace is a Duet capability, not part of the connection: "The workspace is yours and separate from the live connection." Connected care owns only its sending: while connected, the patient may send a saved version to the practice as a deliberate act. There is one Send to my practice, with no separate "look at this" feature. It arrives once, as the version sent, like a message from a known patient. It is never shared by saving and never streamed live. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:15.989Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:36.821Z (chosen); linear:ENG-169#What the patient can do in MVP; linear:ENG-39#Decided]

**Your doctor's plan.** Documented care decisions and open items from the connected practice, each with source, author, date and status. Read-only in Duet. Distinct from patient-owned Plans, which remain the person's routines. After an ending the last plan stays visible "as the patient's own care information, marked no longer updated"; joining again brings it current. [linear:ENG-45#Decided; linear:ENG-45#What the patient can do; linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:55:04.546Z (chosen)]

**Appointment details.** That practice's appointment facts, sent back over the connection and described in the consent preview. They keep their schedule source and never authorize new clinical instructions. They work through updates, cancellation, duplicate reports and disconnection; conflicting dates stay visible and don't block preparation; a suggested follow-up is not presented as a booked appointment. When the connection ends, the practice's appointment goes away and the patient's own appointment date stays. Rejoining clears the old appointment, and the practice's current schedule brings back any visit it has ahead. [linear:ENG-70#The model; linear:Connected care (V1)#MVP outcomes; linear:ENG-169#Complete when — MVP workspace; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:53:13.048Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:09:49.108Z (chosen)]

**Plan Check-in.** A Duet Check-in tied to the doctor's plan. Like any recurring Check-in, it runs only once the patient has accepted it, within its agreed scope. Answers feed the practice's next Agents review; they never become alerts. [linear:ENG-45#What the patient can do; linear:Duet#How Duet's AI works; codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:25:21.679Z (chosen); linear:ENG-14#Decided; linear:ENG-45#Done when]

**Roster status.** The practice's only view of the connection itself: Invited, Connected or Ended on the roster, with no notifications. [linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]

**The AI on each side.** Duet's AI explains the doctor's plan and proposes notes; it shows no activity UI to the patient. The practice's agents review what arrives on their scheduled passes and decide whether a physician needs to see it; only Agents shows run steps. [linear:ENG-45#What the patient can do; linear:ENG-160#Decided; linear:ENG-39#Done when; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:51.881Z]

## 4. Flows

### C1.1 Invitation

1. A practice member invites a patient from that patient's roster row. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:56:50.590Z (chosen); linear:ENG-70#Flows]
2. Coda sends the invite by text first, with email as a backup. [linear:ENG-70#Flows]
3. The practice sees Invited on the roster, with no notifications. [linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]
4. If the patient doesn't respond, Coda sends one reminder, then the invite expires. [linear:ENG-70#Flows]
5. Practice members may resend or cancel the invite at any time it is open. [linear:ENG-70#Flows]
6. Branch, resend in flight: if the patient taps Share on an invite the practice still holds open, they connect and both sides show Connected, even while a resend is under way or couldn't be delivered. [linear:ENG-70#Four parts, each with its own states; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T17:12:54.534Z (chosen)]
7. Branch, patient already connected to another practice: the patient may join this practice too. Each connection is separate, and the existing one is unaffected. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z; linear:ENG-70#Flows]
8. Branch, expired or cancelled link: GAP: what the patient sees when opening an expired or cancelled invite is not stated. [linear:ENG-70#Flows]

### C1.2 Joining with consent

1. The patient taps the invite link. It opens Duet if installed; otherwise it goes to the App Store or Play Store and the invite resumes after install. A short code works as a fallback. [linear:ENG-70#Flows] There is no Duet web surface in the MVP; an invitation link opens the app [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z].
2. The patient signs in, or signs up if new to Duet; invitations use email sign-in, and joining from an invitation links to the account they already have. [linear:ENG-70#Flows; linear:ENG-161#Decided]
3. The patient confirms their date of birth. [linear:ENG-70#Flows]
4. Branch, date of birth doesn't match: wrong guesses lock the code. This was set in the Oct 12 shared-space design and ENG-70 doesn't restate it. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T20:08:54.105Z (chosen); linear:ENG-70#Flows]
5. The patient sees the practice, their physician, and a plain preview of what will be shared, including appointment details coming back. [linear:ENG-70#Flows; linear:ENG-70#The model]
6. The patient taps **Share**. This one act grants consent for the fixed wording version and connects. [linear:ENG-70#Flows; linear:ENG-70#Four parts, each with its own states; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen)]
7. Branch, patient declines: GAP: what happens to the invite and what the practice sees if the patient opens it and doesn't tap Share is not stated. [linear:ENG-70#Flows]
8. Joining shares what the patient already has: their connected records reach the practice with the join, with no further tap. [linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T19:53:52.075Z]
9. The practice's agents start work on those records. [linear:ENG-70#Flows]
10. The roster shows Connected; Duet shows the same status. [linear:ENG-70#Done when (MVP); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]
11. A new patient begins in conversation, with records and doctor preparation as optional next steps. [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:25:41.837Z (chosen)]

### C1.3 Consented records reaching the physician

1. While connected, what the patient connected in Duet reaches the practice continuously: portal records, phone readings, and notes and medications they entered. [linear:ENG-39#What the patient and physician can do; linear:ENG-39#Decided]
2. Duet's AI proposes short dated notes from conversation. The patient reviews and saves a note; only then can it cross. A note or medication the patient asks Duet to save is saved at once and crosses like any entry they made. [linear:ENG-39#What the patient and physician can do; claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T14:56:06.803Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T18:58:16.345Z (chosen)]
3. The AI's private notes about the patient never cross. [linear:ENG-39#Decided]
4. The practice's agents read what arrives and judge whether a physician needs to see it. [linear:ENG-45#Decided]
5. The physician sees Duet data in the patient's chart, reviews, preparation and answers, each item labelled with origin and date, e.g. "from Epic, via the patient's Duet". The physician reads the record exactly as sent. [linear:ENG-39#What the patient and physician can do; linear:coda-rust platform#Journeys]
6. Copies of records the chart or Kno2 already holds arrive labelled and recognizable as duplicates. [linear:ENG-39#Done when]
7. Branch, a record is corrected at its source: the new version arrives as its own event, not a new arrival. Unfinished work doesn't publish from the version it replaced. The practice keeps the earlier version it received, dated. [linear:ENG-39#Done when]
8. The patient sees no "your care team looked" read receipt on shared items. [linear:ENG-45#Decided; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T19:59:48.189Z]
9. Branch, sharing is behind or failing: someone who can act sees it, with the reason. [linear:coda-rust platform#What went wrong before] The patient sees problems with their own sources in Duet; Coda ops sees problems in the sharing pipeline. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T02:07:40.936Z (chosen)]

### C1.4 Documented care decisions reaching Duet

1. The practice documents care decisions in its own record. [linear:Connected care (V1)#MVP outcomes; linear:ENG-45#Decided] What reaches Duet is what is final in the EHR (signed notes, active medications and orders, the problem list) as the scheduled pull reads it. Coda's agents judge what is a care decision for the patient, and unsigned drafts never go. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:11.518Z (chosen)]
2. Documented decisions and open items reach Duet with source, author, date and current status. They are updates, not messages, so no physician approves them; any message to the patient still needs the physician's approval [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z]. [linear:coda-rust platform#Journeys; linear:ENG-45#What the patient can do; linear:Connected care (V1)#MVP outcomes]
3. The patient sees them as "your doctor's plan", read-only, separate from their own Plans. [linear:ENG-45#Decided]
4. The patient asks Duet to explain the plan in plain language; Duet explains it faithfully. [linear:ENG-45#What the patient can do; linear:ENG-45#Done when]
5. Duet keeps a documented decision, an unconfirmed suggestion and a patient report distinct. An undecided medication recommendation stays an unconfirmed suggestion until the source records a decision. [linear:ENG-45#Done when; linear:ENG-45#Don't repeat]
6. Branch, the source is ambiguous: Duet shows the ambiguity rather than turning it into an instruction. [linear:ENG-45#Discuss with Dev when you get here]
7. That practice's appointment details also arrive, keeping their schedule source; they never authorize new instructions. [linear:ENG-70#The model; linear:Connected care (V1)#MVP outcomes]
8. The patient prepares their side of a visit through the doctor workspace (ENG-169). [linear:ENG-45#What the patient can do]
9. Branch, the plan changes: a later documented change updates the patient's view with its provenance. What the patient is looking at and working on takes the newer plan; anything Duet already said or sent keeps the version it used and is marked when a newer one exists. [linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:32:53.088Z]
10. The physician sees nothing new for this step: no Duet approval queue, and physicians don't manage the patient's Duet. [linear:Connected care (V1)#MVP outcomes; linear:ENG-45#Done when]
11. News from the practice (the plan arriving or changing, an invitation, an appointment scheduled, moved or cancelled) alerts the patient with a generic lock-screen line and no health content; tapping it opens it in Duet. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T17:15:08.163Z (chosen)]

### C1.5 Plan Check-ins

1. The patient uses Check-ins related to their doctor's plan. [linear:ENG-45#What the patient can do]
2. A plan Check-in runs only once the patient has accepted it, and adapts only within the limits they accepted. [linear:Duet#How Duet's AI works; codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:25:21.679Z (chosen)] Duet's AI proposes a Check-in from the doctor's plan when useful, and it runs only once the patient accepts. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T02:07:40.936Z (chosen)]
3. The answers reach the practice's next Agents review. [linear:ENG-45#Done when; linear:ENG-14#Decided]
4. They never become individual alerts for the physician. [linear:ENG-14#Decided; linear:ENG-45#Done when]
5. The practice's agents decide whether a physician needs to see anything. [linear:ENG-45#Decided]

### C1.6 Sending For my doctor

1. The patient prepares and saves For my doctor in Duet; the workspace is separate from the connection. Saving shares nothing. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:15.989Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:12:19.370Z; linear:ENG-39#Decided]
2. While connected, the patient taps Send to my practice, with any question for the practice written in it. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:36.821Z (chosen); linear:Connected care (V1)#Features and cadence] Send to my practice names the doctor or practice it goes to. [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-22T20:50:53.402Z]
3. Each record in it goes at its current version: a record corrected at its source says it was corrected and when, and Send is never blocked. Dev rejected sending the old version: "why would we allow legacy record sharing?" [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:29:15.087Z; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:29:38.568Z (chosen)]
4. It reaches the practice once, as the version sent, already linked to the patient like a message from a known patient, and flagged for the practice's agents. [linear:ENG-39#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:18:05.292Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:36.821Z (chosen)]
5. Duet picks it up and hands it to Coda Agents: the patient's agent talks to the practice's AI care team agent. Dev: "You submit things and duet picks it up and transfers it over to CODA agents ... the patient agent communicates with the physician AI care team agent." This matches the Oct 26 choices above. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
6. The practice's agents read it and judge whether a physician needs to see it; if so, it goes on the physician's Today. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:18:05.292Z (chosen); linear:ENG-39#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T20:00:37.921Z]
7. Anything that goes back to the patient as a message, such as an answer to their question, needs the physician's approval (B1). [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z] The approved answer reaches Duet as a message from the practice in the thread, like other practice news. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T02:07:40.936Z (chosen)]
8. The patient sees no receipt that the care team looked, and Duet's wording doesn't claim a physician reviewed it. [linear:ENG-45#Decided]
9. The patient later edits their For my doctor: the sent copy keeps its version and is marked when a newer one exists. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:32:53.088Z]
10. Branch, not connected: Without a live connection there is no Send to my practice; unsupported features are absent from the app (outside-Coda sharing is post-MVP). [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T02:07:40.936Z (chosen)] [linear:ENG-169#What the patient can do in MVP]

### E1. Patient leaves

1. The patient leaves the practice in Duet. [linear:ENG-70#Flows] GAP: where the leave control lives after the Shared tab is removed is open (ENG-141; DECISIONS.md UI5).
2. GAP: whether Duet confirms before leaving and what it says is not stated. [linear:ENG-70#Flows]
3. The connection ends everywhere at once and new data stops. [linear:ENG-70#Done when (MVP); linear:ENG-39#Done when]
4. The patient keeps their Duet account and history. [linear:Connected care (V1)#Current MVP scope]
5. The last doctor's plan stays visible as the patient's own care information, dated, marked no longer updated. The practice's appointment goes away; the patient's own appointment date stays. [linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:55:04.546Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:53:13.048Z (chosen)] Practice news already in Duet's thread stays there as history, and nothing updates after the connection ends: "history stays and nothing gets updated" [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z].
6. The practice keeps the dated copies it already received; the roster shows Ended, with no notification. [linear:ENG-70#Done when (MVP); linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]
7. Short of deleting the account, leaving is the only way to stop sharing in the MVP. [linear:ENG-70#MVP scope; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]
8. GAP: what happens to work the practice's agents were doing on this patient's Duet data when the connection ends is not stated. [linear:ENG-39#Done when]

### E2. Practice discharges the patient

1. A practice member discharges the patient from the patient's roster row and confirms. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:56:50.590Z (chosen); linear:ENG-70#Flows]
2. The connection and data flow end immediately. [linear:ENG-70#MVP scope; codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T03:10:03.993Z (chosen)]
3. Duet shows the patient a dated notice. [linear:ENG-70#Flows] GAP: the notice's content (reason, who to contact) is not stated.
4. The practice's own notice obligations don't extend Duet's sharing permission. [linear:ENG-70#MVP scope]
5. The patient keeps their account and history; the last doctor's plan stays visible, marked no longer updated; the practice's appointment goes away. [linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:55:04.546Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:53:13.048Z (chosen)] Practice news already in the thread stays as history; nothing updates after the end [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z].
6. The practice keeps the dated copies it already received; its roster and Duet show the same Ended status. [linear:ENG-70#Done when (MVP)]

### E3. Account deletion

1. The patient deletes their Duet account. [linear:ENG-70#Flows]
2. All connections end and sharing stops at once. [linear:ENG-70#Flows; linear:ENG-39#Done when; linear:ENG-161#Done when]
3. Each practice keeps what it already received, dated. [linear:ENG-70#Flows; linear:ENG-70#Done when (MVP)]
4. The practice roster shows Ended. [linear:ENG-70#Done when (MVP); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)]

### E4. Physician leaves the practice (not an ending)

1. The physician's panel is reassigned; the connection continues. [linear:ENG-70#Four parts, each with its own states]
2. Telling the patient who their doctor is now is later work. [linear:ENG-70#Flows]

### R1. Re-invitation and rejoining

1. After a connection ended, whether the patient left or the practice discharged them, any practice member can send a new invite at any time: "Ended is like never connected for inviting." [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen); linear:ENG-70#MVP scope]
2. The patient must accept and consent again, through C1.1 and C1.2. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen); linear:ENG-70#Flows]
3. Joining again shares everything the patient already has, including what they added while disconnected; the practice's agents start on it. [linear:ENG-70#Flows; linear:ENG-39#Decided; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T19:53:52.075Z]
4. The doctor's plan becomes current again. Rejoining clears the old appointment; the practice's current schedule brings back any visit it has ahead. [linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:09:49.108Z (chosen)]
5. GAP: whether the practice sees the disconnected period labelled is not stated. [linear:ENG-39#Done when]
6. GAP: whether the patient can decline future invites is not stated. [linear:ENG-70#Flows]

### What each side sees

| Step | Duet patient sees | Practice / physician sees |
|---|---|---|
| Invite sent | Text (email backup) with a link or short code [linear:ENG-70#Flows] | Invited on the roster, no notifications [linear:ENG-70#Flows; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen)] |
| Reminder / expiry | One reminder, then expired [linear:ENG-70#Flows]; GAP: expired-link screen | Status; resend or cancel [linear:ENG-70#Flows] |
| Join | DOB check, practice, physician, plain preview, **Share** [linear:ENG-70#Flows] | Connected; existing records arrive and agents start [linear:ENG-70#Flows] |
| Live sharing | No activity UI and no "care team looked" receipt [linear:ENG-160#Decided; linear:ENG-45#Decided] | Items labelled with origin and date in the chart, reviews, preparation and answers; duplicates labelled [linear:ENG-39#What the patient and physician can do; linear:ENG-39#Done when] |
| Doctor's plan | "Your doctor's plan": source, author, date, status; plain-language explanation [linear:ENG-45#What the patient can do] | Nothing to approve [linear:Connected care (V1)#MVP outcomes] |
| Plan Check-in | The accepted Check-in [linear:ENG-45#What the patient can do; linear:Duet#How Duet's AI works] | Answers inside the next Agents review, never an alert [linear:ENG-14#Decided] |
| For my doctor | Send to my practice; records at their current version; no receipt [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:36.821Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:29:38.568Z (chosen); linear:ENG-45#Decided] | Arrives once like a message from a known patient; on the physician's Today only if the agents judge a physician needs it [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:18:05.292Z (chosen)] |
| Leave | Account, history, last plan marked no longer updated; practice appointment gone [linear:Connected care (V1)#Current MVP scope; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:53:13.048Z (chosen)] | Ended; dated copies kept [linear:ENG-70#Done when (MVP)] |
| Discharge | Dated notice; account, history, last plan kept [linear:ENG-70#Flows] | Ended; dated copies kept [linear:ENG-70#Done when (MVP)] |
| Account deletion | Account gone (GAP: confirmation copy) [linear:ENG-161#Done when] | Ended; copies kept [linear:ENG-70#Flows] |
| Physician leaves | Later: who their doctor is now [linear:ENG-70#Flows] | Panel reassigned [linear:ENG-70#Four parts, each with its own states] |
| Re-invite / rejoin | New invite, consent again; plan current again; old appointment cleared [linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:09:49.108Z (chosen)] | Connected; existing data arrives again [linear:ENG-70#Flows] |

## 5. Rules

1. Nothing crosses without a current connection and current consent. [linear:ENG-39#Done when; linear:ENG-70#Four parts, each with its own states]
2. One consent record per connection decides everything that crosses; only the patient changes it. [linear:ENG-70#Done when (MVP)]
3. Connected means sharing: there is no paused state, and the roster shows only Invited, Connected or Ended. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:57:20.608Z (chosen); linear:ENG-70#MVP scope]
4. Ending the connection in Duet ends it everywhere; patient leave, discharge and deletion stop new data at once. [linear:ENG-70#Done when (MVP); linear:ENG-39#Done when]
5. Duet and the practice always show the same connection status. [linear:ENG-70#Done when (MVP)]
6. Any practice member may invite, re-invite or discharge; discharge asks for confirmation and takes effect immediately. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T14:56:50.590Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen)]
7. The practice keeps dated copies of what it already received after any ending. [linear:ENG-70#Done when (MVP)]
8. Ending preserves the patient's Duet account and history, including practice news already in the thread, and the last doctor's plan, marked no longer updated. Live practice items stop: the practice's appointment goes away, and nothing updates after the end. [linear:Connected care (V1)#Current MVP scope; linear:ENG-45#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:53:13.048Z (chosen); claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
9. On discharge, data flow stops immediately and Duet shows a dated notice; the practice's notice obligations don't extend sharing. [linear:ENG-70#MVP scope]
10. Notes from conversation cross only after the patient saves them, by asking Duet to save one or approving one Duet proposed; the AI's private notes never cross. [linear:ENG-39#Done when; claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T14:56:06.803Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T18:58:16.345Z (chosen)]
11. For my doctor crosses only when the patient sends it, through one Send to my practice, once, as the version sent; each record in it goes at its current version, marked if corrected. [linear:ENG-39#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T02:14:15.989Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T21:29:38.568Z (chosen)]
12. Every item that crosses keeps its origin and date. [linear:ENG-39#Decided]
13. A corrected record is a new version and its own event; unfinished work never publishes from the replaced version. [linear:ENG-39#Done when]
14. Duplicates of records the chart or Kno2 already holds arrive labelled as duplicates. [linear:ENG-39#Done when]
15. Physicians don't approve Duet's wording or manage Duet; Duet's wording never claims physician review. [linear:ENG-45#Decided]
16. Duet never invents or amends care instructions; an undecided recommendation stays an unconfirmed suggestion. [linear:ENG-45#Done when; linear:ENG-45#Don't repeat]
17. Appointment details keep their schedule source and never authorize new clinical instructions. [linear:ENG-70#The model]
18. Plan Check-in answers never become alerts. [linear:ENG-14#Decided]
19. No "your care team looked" read receipt on shared items. [linear:ENG-45#Decided]
20. Each request sees only its own person's data. (doc) [doc:coda-app/behaviour-contract.md:209]
21. Adults 18+ only. [linear:ENG-70#Not in V1]
22. Every MVP feature is free; Agents never knows who pays and doesn't manage billing. [linear:Connected care (V1)#Features and cadence; linear:Coda Agents#Duet–Agents connection]

## 6. Scope

**MVP in**
- Practice connections, each reached by invitation, with fixed consent at join, sharing until patient leave, practice discharge or account deletion. [linear:coda-rust platform#Scope; linear:ENG-70#MVP scope]
- A patient may hold connections to more than one practice, each separate. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- Live sharing of what the patient connected; approved notes; For my doctor send. [linear:ENG-39#What the patient and physician can do; linear:ENG-39#Decided]
- Your doctor's plan (read-only) with plain-language explanation and plan Check-ins. [linear:ENG-45#Decided]
- Ending a connection with account and history preserved. [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:25:21.679Z (chosen); linear:Connected care (V1)#Current MVP scope]
- Re-invitation by any practice member after an ended connection. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen); linear:ENG-70#MVP scope]
- Physician on Coda Agents web; patients on Duet iPhone and Android. [linear:coda-rust platform#The MVP] Duet has no web surface; invitation links open the app. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- Free for every connected patient. [linear:ENG-39#Decided; linear:coda-rust platform#Scope]
- General phone-health ingestion and daily portal refresh come before CGM; CGM doesn't gate sharing. [linear:ENG-39#Integration order]

**Out of MVP / later**
- Stop sharing and resume; withdrawal; re-consent after a wording change. [linear:ENG-70#Not in V1; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T16:03:24.772Z (chosen)]
- Patient-started connections through clinician lookup; care-team saving. [linear:Connected care (V1)#Current MVP scope; codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:25:21.679Z (chosen)]
- Sharing with doctors outside Coda: browser review, no PDF download, selected records with a short overview. [linear:Connected care (V1)#Current MVP scope; codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-23T00:10:33.694Z (chosen); claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T02:34:53.637Z]
- Caregivers and proxies; minors. [linear:ENG-70#Not in V1]
- Telling the patient their new doctor after a physician leaves. [linear:ENG-70#Flows]
- Release of information. [claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-23T01:41:08.694Z; linear:ENG-183#Scope decision]
- Paid tier, pricing and gating; premium will belong to the connection, and "nothing clinical is paywalled". [linear:Duet#What Duet is; linear:Duet#The rulebook; linear:Connected care (V1)#Features and cadence]
- No "coming soon" entry for any post-MVP connected feature; unsupported features are absent from the app. [linear:Duet#Current MVP scope; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]

**Ideal, not MVP: the ENG-27 shared space (background design, no build commitment)**
- A jointly maintained page per patient–physician relationship; two-way replies and co-editing. [linear:ENG-27#header note; linear:ENG-45#Decided] The practice invites, as in the MVP. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T20:07:14.745Z (chosen)]
- Two people only until the core works. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T19:43:18.565Z (chosen)]
- "Each side's agent drafts and its person confirms." [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T19:40:49.258Z (chosen)]
- The space needs the physician on Coda Agents; specialists not on Coda get a read-only view grown out of For my doctor. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T19:59:25.003Z (chosen); linear:ENG-27#Decided (Dev, 2026-10-12)]
- Plan confirmation is a signal, not a gate: "Maria's 'looks right' covers the whole plan in one tap." [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T20:10:44.949Z (chosen)]
- "Urgent symptoms send her straight to call the practice or 911, and never wait in the space." [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T22:43:52.757Z (chosen)]
- Coda doesn't answer the patient from standing rules: every message from the practice to the patient needs the physician's approval (B1, which settles DECISIONS.md L1). [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z]

## 7. Harness boundary

Events and guarantees Connected care relies on. No runtime design.

1. Events both products share: invitation (sent, reminded, accepted, expired, cancelled), join, leave, discharge, account deletion, the live feed of shared items, and a sent For my doctor. [linear:ENG-185#What the graphs cover; linear:ENG-70#Four parts, each with its own states; linear:ENG-39#Done when]
2. Connection status has one owner that both sides read. [linear:ENG-70#Done when (MVP); linear:coda-rust platform#What went wrong before]
3. An ending takes effect at once: no new item crosses after it. [linear:ENG-39#Done when; linear:ENG-70#MVP scope]
4. A sent For my doctor is delivered exactly once, as the version sent. [linear:ENG-39#Done when]
5. A record correction is its own event and reaches unfinished work before it publishes. [linear:ENG-39#Done when; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-25T19:53:52.075Z]
6. A join delivers the patient's existing records and starts the practice's agents. [linear:ENG-70#Flows]
7. Plan Check-in answers are delivered into the next Agents review, not into alerting. [linear:ENG-14#Decided]
8. Failed or slow pulls retry, then degrade honestly with the date of what is shown; repeated failure becomes visible to someone who can act. [codex:rollout-2026-09-09T22-56-37-01a0893e-ecdd-7212-b390-c0ef653d@2026-09-11T01:25:39.366Z; linear:coda-rust platform#What went wrong before; claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-24T23:19:33.482Z (chosen)]
9. Neither product reads the other's storage; data crosses only through the connection. [linear:Connected care (V1)#Features and cadence; doc:coda-app/README.md:59]
