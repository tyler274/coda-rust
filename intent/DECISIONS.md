# Decisions for Dev

One place for every open question and every settled conflict across physician.md, duet.md, connected-care.md and harness.md. The specs carry no open questions or conflicts of their own. Every quote is verbatim from its source. Pointers follow the specs: `claude:<session>…@<UTC time>` and `codex:…<session>…@<UTC time>` are Dev's words, and `(chosen)` marks an option Dev clicked. `linear:` is Linear as read on 2026-10-28; a Linear line with no date of its own is marked "undated". `(doc)` and `(site)` are not Dev's words.

Answer by ID, for example "B1: A" or "S4: recommendation".

---

## Open for Dev

All 28 contradictions from the sweep are settled, by your answers, your later words or the principles; they're listed under Decided by Dev and Already settled. The questions below aren't contradictions. They are things no source answers yet, gathered here from the four specs so there is one list. The specs no longer carry their own.

### For Dev, in the MVP

Nothing open. O1–O7 were answered on Oct 28 (Decided by Dev).

### For Tyler (runtime and architecture)

- **T1. Production triggers and the schedule source.** How scheduled and pre-visit triggers run in production, and the real schedule source. The agents judge which visits to prepare and how far ahead (PRINCIPLES.md 4). [codex:…01a081e0…@2026-09-10T14:31:19; codex:…01a0602a…@2026-09-02T19:36:35] [linear:ENG-31#Discuss with Dev]
- **T2. The patient roster.** How the active roster is kept, likely from the EHR's patient list (with ENG-57). [codex:…01a0602a…@2026-09-03T14:27:48; codex:…01a0602a…@2026-09-03T14:31:55]
- **T3. Freshness after a one-time pull.** How records from other systems stay fresh. Every source shows when it was last refreshed (Freshness, decided). [codex:rollout-2026-09-02T23-10-23-01a0653f-0357-72a0-975d-7d195fe3@2026-09-03T14:25:44.920Z]
- **T4. athena.** The patient-access route and the remaining gap. [linear:ENG-162#Discuss with Dev when you get here]
- **T5. Push registration.** Fix the security review's push problem now that notifications follow whoever is signed in. [linear:ENG-34#Discuss with Dev when you get here; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T17:15:32.634Z]
- **T6. Chart search.** Outline first or plain search, settled by evals. [claude:220b86fb…@2026-09-24T17:09:14; claude:220b86fb…@2026-09-24T17:46:31]
- **T7. Browser work.** Luna's own browser loop as primary and native computer use as fallback; your Sept 30 leaning, not decided. [codex:…01a0efb2…@2026-09-30T02:23:52]
- **T8. Anthropic BAA.** It exists, but no source uses it for product calls; Linear names the OpenAI API. [codex:rollout-2026-10-16T23-44…@2026-10-17T16:54:06; linear:ENG-160#Decided]
- **T9. Prompt injection.** Named as a capability lane, with no design recorded. [codex:…@2026-09-19T00:52:39]
- **T10. The simulation's clock.** What "today" means once it moves. [claude:9278ff48…@2026-10-26T17:43:45]

### Studies you asked for

- **R1. Regexes.** List the product regexes that infer meaning from free text or model output, with keep or remove for each. [claude:220b86fb…@2026-09-28T20:21:09]
- **R2. CDS compliance.** Whether a lightweight deterministic contract should back the prompts. [codex:…01a091f9…@2026-09-14T15:02:08; claude:f015cdb7…@2026-10-01T21:21:28]

### With the UI program (judged against PRINCIPLES.md U1–U9)

- **UI1. Today's composition.** Order and density of insights, appointments, Plans, Check-ins, the doctor's plan, For my doctor and measurements, and whether another overview adds anything. [linear:ENG-168#Discuss with Dev when you get here]
- **UI2. Entry and return.** Where the contextual entry and return controls sit between Today and chat. [linear:ENG-168#Discuss with Dev when you get here]
- **UI3. Saved forms.** Which outputs beyond Plans, Check-ins and For my doctor need a saved, editable form. [linear:ENG-168#Discuss with Dev when you get here]
- **UI4. Uncertain appointment dates.** How they look; Duet's AI judges when a reminder about them is useful. [linear:ENG-169#Discuss with Dev when you get here]
- **UI5. Connected care controls.** Their patient-visible home (join status, leave) once the Shared tab is gone. [linear:ENG-141#Discuss with Dev when you get here]
- **UI6. Physician Delete.** Is Delete a conversation control beside Archive? The docs describe only Archive and Restore. [codex:…01a091f9…@2026-09-11T19:42:47; coda-agents/docs/physician-experience.md:372]
- **UI7. Where the Updates log lives.** Today ends with the collapsed "Coda also reviewed…" line. Whether the full log also keeps its own page, and how it looks, is decided from a mockup during the port. What goes on Today, in Updates or on the patient page is Coda's judgment, not a UI rule. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:30:36.402Z]
- L2 (site) and L3 (companion) below also wait for the UI program.

### After the MVP

- **A1. Coda's own clinical record.** Whether Coda keeps its own record or builds into EHRs. In the MVP the EHR is the source of truth, read only (E1), and Duet data a practice receives lives in Coda (B2). [codex:…01a0f98d…@2026-10-09T20:21:01] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:12:44.566Z]
- **A2. Who may send** once Coda can send; practice roles aren't built. (doc) [coda-agents/docs/patient-messaging.md:1146]
- **A3. The patient as co-owner of the record.** Still being explored. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T17:21:24.135Z]
- **A4. Duet calendar and email connections.** [linear:ENG-169#Discuss with Dev when you get here]
- **A5. NPI registry ingestion** as separately scoped groundwork. [linear:ENG-183#Intended later experience]
- **A6. Premium and pricing.** Duet stays free until you pick pricing (Q10). The standing bound is that premium only includes what exists because the patient has a doctor on Coda. [claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T17:31:44.942Z; claude:1832669a-01e1-4365-916f-ae03e08161f4.jsonl@2026-10-20T20:42:55.700Z] [claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T18:48:21.355Z (implied)] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z (chosen)]
- **A7. A Hint route** that doesn't depend on the practice's open Chrome. [linear:ENG-37#Discuss with Dev] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:34:59.355Z] [linear:ENG-64#open-brainstorm-with-dev-before-building]
- **A8. ENG-159 boundary changes.** Changing clinical authority, sharing product state, unifying the products' context or saving policies, or exposing execution reasoning in Duet would each be a new decision. [linear:ENG-159#Discuss with Dev when you get here]
- **A9. Background-run limits.** No limit in the MVP. Later there may be a limit on total background runs, such as a cap on runs per patient by what the practice pays. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z]
- **A10. Reference ranges on For my doctor.** Not in the MVP; ideally later, once their sourcing is better. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:38:41.451Z]

### Before production

- **BP1. Epic questionnaire and privacy policy.** Correct the draft Epic Data Use Questionnaire and the privacy wording to match Duet D2.15 (C2). [codex:rollout-2026-09-05T10-36-49-01a07200-2c17-78d0-aac6-b0f0cec0@2026-09-05T14:36:52.863Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:29:06.679Z (chosen)]

---

## Later / ideal-state

### L1. In the ENG-27 shared space, does the physician approve every reply to the patient? Settled by B1.
- **Yes, for anything the practice sends.** B1 (Oct 28): every message that goes to the patient needs the physician's approval. "Physicians do not approve or manage Duet for patients" [linear:ENG-45#Decided] is about Duet's own AI, which isn't the practice, so the two don't conflict. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z; linear:ENG-27#The idea in brief]

### L2. Should the public site match the app? Settled: the specs follow the app only.
- The website is updated later and doesn't govern; where it differs from the app, the app wins. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:38:41.451Z; claude:a55fdd59…@2026-10-22T00:19:13]

### L3. Does the companion survive, does the chat header show its name, and are its one-time "relationship moments" (approved 10-02) wanted?
- **A. The companion names the chat.** The chat header shows the companion's name: "Show companion's name" [claude:04e60e17…@2026-09-28T03:48:15 (chosen)], 2026-09-28.
- **B. The chat is Duet's AI.** "rename Luna to Duet's AI" [claude:1832669a…@2026-10-20T21:14:08], 2026-10-20; Plans and Check-ins move "out of the companion card" [linear:ENG-34#What the person can do].
- **Specs:** duet (Companion).
- **Recommendation:** decide with the UI program. If the companion stays, its name stays on the face and the chat stays Duet's AI.

---

## Decided by Dev

- **B1 (Oct 28).** "Any message that goes to the patient, like the communication needs to be approved by the physician, but updates that we notice don't need to be approved." Every message to the patient needs physician approval; documented care decisions reaching Duet are updates and need none. Updated: physician principle 2, connected-care C1.4. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z]
- **B3 (Oct 28).** "my view is we could have multiple models running for accounts." Runs may be parallel within an account, without brittle limits; Tyler decides the mechanism. ENG-159's "one in-flight paid model run per account" needs Tyler's review. Updated: duet section 7, harness G4. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:11:28.018Z]

- **B2 (Oct 28).** "Yes, in Coda": Duet data a practice receives lives in Coda's record for that patient, never written to the EHR. Updated: connected-care principle 2, physician open questions. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:12:44.566Z]

- **S1 (Oct 28).** Settled, consistent with the Oct 26 choices. The patient sends For my doctor or a question; Duet picks it up and hands it to Coda Agents, where the patient's agent talks to the practice's AI care team agent and the practice's agents judge what the physician needs to see. Any message back to the patient needs physician approval (B1). "the patient agent communicates with the physician AI care team agent." Updated: connected-care C1.6, physician P3, duet D4.7. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S2 (Oct 28).** History stays in the thread; nothing updates after a connection ends. "history stays and nothing gets updated". Updated: connected-care E1, E2, rule 8; duet principle 14, The conversation. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S3 (settled part) (Oct 28).** Ingesting Duet data properly comes first; review cadence and content were then settled by the review model below. Updated: physician Cadence, connected-care and duet review. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S4 (Oct 28).** "if we can figure it out": cancelling or moving a visit ends unfinished preparation at once; a finished briefing stays as history with a notice. Updated: physician P4. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S5 (Oct 28).** "anything, it's free. It's literally a powerful model." Duet's own work may take on anything; the harness guides and empowers and never restricts. The person still controls what is saved, reviewed or shared. ENG-34's "perform only their accepted work" needs updating in Linear. Updated: duet principle 4, Duet's own work; PRINCIPLES.md 2. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S7 (Oct 28).** "No, the MVP should not just allow one connection." A patient may connect to more than one practice, each separate. ENG-70, ENG-183, the Connected care project and the platform scope need updating in Linear; the platform's "one live practice" is read as one practice live at launch (an interpretation). Updated: connected-care Connection, C1.1, Scope; duet Connected care, Scope. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S8 (Oct 28).** A push only wakes the work; the work always pulls current truth. Updated: harness principle 6, physician section 8, connected-care live feed, duet section 7. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S9 (Oct 28).** "conversation is own thing, background has its own thing"; Sol (conversation) and Luna (background) are the actual models, not configuration names. Duet's model is "Duet's AI" in Duet product language. Updated: physician model roles, HISTORY.md (harness 6), duet Duet's AI. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S10 (Oct 28).** "it's like stop": Discuss holds running work like Stop. Updated: physician K1, P5. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S11 (Oct 28).** No alarm. An urgent finding is a now concern on Today, possibly "some banner at the top that we had played with previously". Updated: physician P2, safety wording. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S12 (Oct 28).** A person can delete a single chat message, like iMessage. Direction note: find an open-source, delightful chat implementation so it feels like texting. Updated: duet The conversation, Chat behavior 7. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S13 (Oct 28).** "records are just like a data dump" for now; no model organization. Updated: duet You, Scope. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S14 (Oct 28).** Unsupported features are absent from the app. ENG-23's "stated" line needs aligning. Updated: duet Scope, connected-care Scope. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S15 (Oct 28).** Fail closed on access, consent and patient data; fail gracefully on availability and single bad items. Updated: physician principle 14, harness principle 5, duet Consent and data. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]
- **S16 (Oct 28).** No Duet web surface in the MVP; invitation links open the app. ENG-161's "email on the web" needs aligning. Updated: duet D1.2, Scope; connected-care C1.2, Scope. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:24:59.542Z]

- **Duet principle (Oct 28).** "the core guiding principle on the product is this is instinct or muse or grockbot or hermez or open claude managed and like maintained for you. For health, right? the, in my view, the today page, all that stuff is just effectively, a layer and interface and, tool calls". Duet is a managed health agent. Today, Plans, Reminders, Check-ins, You and For my doctor are views of what it keeps, written by its tool calls; the person changes them by telling Duet or with a direct shortcut, and Duet handles consequences with judgment, not dialogs. Updated: duet principle 1 and mental models. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:08:20.139Z]
- **C1 (Oct 28).** One AI permission; no separate pause for Duet's own work. Updated: duet mental models, D3. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:10:55.899Z]
- **C3 (Oct 28).** Removing a Plan never stops its accepted Reminders or Check-ins; no dialog; Duet follows up in the thread when useful. Updated: duet D3.23. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:10:55.899Z]
- **C4 (Oct 28).** "Yeah, read only access for now." Gmail Connect ships with read-only Google access and isn't gated on sending; compose comes with Send. Updated: physician P3. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:08:20.139Z]
- **C5 (Oct 28).** "Scribe is not going to be in the MVP". Updated: physician Scope. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:08:20.139Z]
- **C6 (Oct 28).** Agents may send patient-specific questions to outside clinical AI vendors under a BAA, as with OpenAI; Coda's own library stays the default. Stale: coda-agents/docs/architecture.md:1201-1203. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:10:55.899Z]
- **C7 (Oct 28).** The patient summary starts open. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:10:55.899Z]
- **Q6, Q7, Q9 (Oct 28).** An own-work alert opens its message in the thread. Deleting a chat message removes only the message, and Duet's memory is corrected by telling it. The 5-minute no-review window is dropped, and a new message steers. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:10:55.899Z]

- **C2 (Oct 28).** Disconnecting a Duet source keeps the imported history and stops new imports; the person can still delete it, or the account. The Epic questionnaire draft and privacy policy get corrected before production. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:29:06.679Z]
- **Q1–Q3 (Oct 28).** The clinical library, Kno2 outside records (read only), and OCR of scans and faxes are all in the MVP. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:29:06.679Z]
- **Q4 (Oct 28).** The Filtered list is in the MVP: "It should work through natural language or through, yeah, that filtered list in like a nice, graceful UI". [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z]
- **Q10 (Oct 28).** There's no patient-billing goal until you pick pricing. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z]
- **Q12 (Oct 28).** Duet uses the clinical library only, with no web search: "it would be very difficult to prove that it de-identified queries properly." The research backs this. Health AI covered by a BAA doesn't send queries to outside search engines; ChatGPT for Healthcare searches OpenAI's own index. A search index covered by a BAA is the later path. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z]
- **S3 and the review model (Oct 28).** "arrivals shouldn't wake... We'll have chron jobs that handle the proactive review." [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:51.881Z] For every data source (EHR, Kno2, Duet), proactive review runs on a schedule and pulls; patient mail is still handled when it arrives ("Scheduled, except patient mail"). Frequency is Tyler's runtime budget. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:34:59.355Z]
- **Freshness (Oct 28).** Every source shows when its records were last refreshed; this isn't specific to Hint, which is read through computer-use exports. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:34:59.355Z]

- **Context management (Oct 28).** The old runtimes' Codex-based design is ported. It uses skills, system prompts and progressive disclosure, never stuffs context, and doesn't use subagents unnecessarily. Skills improve from traces, and what nearly every run needs is promoted into the resident prompt. Updated: harness principle 1 and Context management, PRINCIPLES.md 2. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:46:26.476Z]

- **Q11 (Oct 28).** Re-engagement: Duet reaches out only for a real reason in the person's life (new records, an appointment, a trend, practice news, or something they were working on), never a generic nudge. It backs off by judgment when unanswered and catches them up when they return. The in-app "Insights and reviews" switch is dropped; the phone's permission is the control. App only in the MVP, with no email or SMS. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:50:24.500Z (chosen)]

- **E1 (Oct 28).** "Yes, read-only MVP". MVP EHR access is read only, and the physician-approved unsigned draft note is the first write after the MVP. Updated: physician section 7, Write-back. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:51:25.837Z]

- **O1–O7 (Oct 28).**
  - O1: the MVP library is the documented PMC set plus BAA-covered outside clinical AI; licensed content joins when a contract lands, and nothing waits on it. Updated: physician Scope, duet library rule. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:53.760Z (chosen)]
  - O2: physicians see live steps from real tool calls, not the model's reasoning; reasoning summaries stay in the team inspector. Updated: physician P1.7. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:11.518Z (chosen)]
  - O3: invite-only. Coda sets up each practice and invites its first physician, and members invite colleagues; no public sign-up. Updated: physician A1.5. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:11.518Z (chosen)]
  - O4: what is final in the EHR (signed notes, active meds and orders, the problem list) reaches Duet as the doctor's plan; agents judge what is a decision, and drafts never go. Updated: connected-care C1.4. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:11.518Z (chosen)]
  - O5: Duet asks for every Health Connect type Google approves, and the person grants each. Updated: duet D2.18. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:11.518Z (chosen)]
  - O6: clinical advisors first, then pilot-practice physicians, review representative For my doctor summaries; no approval step. Updated: duet D4.13. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T21:06:53.760Z (chosen)]
  - O7: settled by your Oct 22 words. The decisions API idea sits with ENG-183 NPI matching, after the MVP; MedDecider has no MVP role, and model choice is Tyler's. Updated: duet Scope. [codex:rollout-2026-10-21T16-08-06-01a16014-0209-7d01-a2bd-e5b72d31@2026-10-22T19:46:29.608Z]
- **Physician walkthrough (Oct 28, evening).**
  - The goal is the website's: Coda connects the dots across every data point, and the EHR is one input. Updated: physician Purpose. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z]
  - Today shows the most salient things. Coda decides every placement (Today, Updates, patient page); only access, never-silent and physician approval are fixed. Today ends with a collapsed "Coda also reviewed…" line; whether Updates keeps its own page is UI7. Updated: physician Today, Updates, P3, P5. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:30:36.402Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:36:25.855Z]
  - Basis and limitations stays on every response, collapsed, for FDA reasons. Updated: physician Mental models. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z]
  - Physicians don't change the full background cadence but can ask Coda for specific scheduled jobs; no run limit in the MVP (A9). Updated: physician Cadence, A1.12. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z]
  - Patient mail is the MVP scope; colleague and insurer mail is outside it. Updated: physician P3.6. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:36:25.855Z]
  - Reference ranges stay off For my doctor in the MVP, ideally later (A10). Updated: duet Rules. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:38:41.451Z]
  - The specs follow the app; the website is updated later (L2). [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:38:41.451Z]
  - Background runs will probably use GPT-6 Luna Max in both products, but it's flexible. Updated: physician model roles, duet Duet's AI. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:44:31.878Z]

## Already settled (confirm)

A later dated Dev statement or Linear decision settles each of these. The specs follow the later source; confirm or reopen.

1. **Quiet work and Updates**. ENG-14 says both "Quiet completion is a valid outcome and creates no routine receipt" and "Work that ends with nothing left to do is logged in Updates with its reason (Dev, 2026-10-23)". An Updates log line is not a receipt. physician.md principle 6 now uses Linear's wording.
2. **Old docs defer Duet data in Coda**. This is superseded by "The connection is live ... (Dev, 2026-10-25)" [linear:ENG-39#Decided].
3. **Phase 1 not owning the patient experience**. Dev, 09-21: "we do't need to own patient experience". This is superseded by the workspace being in MVP (Dev, 2026-10-24) [linear:coda-rust platform#Scope].
4. **The FDA rationale for physician approval**. ENG-27's rationale and Dev's "we need to eb careful with fda regulations" (10-12) are superseded by ENG-45 (Dev, 2026-10-21): no approval. Linear keeps "Clinical and regulatory qualification of the actual patient-facing functions" in real-patient readiness [linear:Duet#Patient-facing help].
5. **Harness as "one fucking central owner"** (harness, Oct 8). This is superseded by the ENG-159 boundary (Dev 2026-10-20, reaffirmed 2026-10-27).
6. **"When and how often Duet looks becomes harness"** (Oct 26 choice). This means outside the behavior graph. ENG-159, reaffirmed 2026-10-27, keeps scheduling, memory and recovery product-owned. duet.md now says "Duet-owned runtime policy". The memory line had cited Dev's question as a decision; it now quotes the question.
7. **AI off as a kill switch**. The 09-23 "Nothing new is published" choice is superseded by ENG-34: off "pauses AI work without using it up". ENG-34 is undated but written for the Rust app; confirm. What remains open is C1.
8. **Citations marked "unavailable"**. This is not a conflict. On 09-27 you chose that previews are required for every cited record, which is the stronger form of your 09-18 point. "Or an unavailable mark" was the agent's wording. Every cited record Coda read gets its preview; "unavailable" applies only to a source Coda couldn't read at all.
9. **The sharing choice when linking an account**. This is settled by your 09-16 18:11 "I agree with your recommendation", which removed the linking-time choice. Linked messages are an ordinary patient source.
10. **The scribe in V1**. This is now C5.
11. **Context checkpoints**. Dev, 09-24: "they should always be secodnary". This is superseded by ENG-159 (2026-10-20), which keeps physician model-written window checkpoints.
12. **Daily re-pull vs configurable cadence** (physician mental models). Your 09-28 pick chose a method: full re-pull rather than vendor change feeds. On 10-26 you said "Keep cadence configurable". So full re-pull is the method, and the cadence is configurable, starting at daily. A record missing from a re-pull is noticed, never deleted on absence alone.
13. **Code deciding evidence is "unchanged"** (harness, What the harness must never do). Doc vs Linear: ENG-14's source checks "start a full review only when something changed". harness.md now scopes the doc line.
14. **The site showing share-sheet sharing**. Dev, 2026-10-22: "site can run ahead of the app".
15. **Notes the patient asks Duet to save** (connected-care vs duet). The 10-26 "Do it, then show" choice satisfies ENG-39's rule, "A note from conversation crosses only after the patient saves it". connected-care.md now covers both paths.
16. **The run baseline vs news mid-run** (harness doc vs products). ENG-160 (Dev, 2026-10-27) takes in news before a run finishes and records the exact versions read. harness.md now says so.
17. **The model setting its own later work**. On 09-04 you said yourself "I guess we never talked about that". Since then your words consistently favor it, through S5 "anything, it's free". The model may set its own later work, through follow-ups and Duet's own work.
18. **How running work shows** (physician UI). Settled by your 10-02 pick, "Three pulsing dots". Details wait for the UI program.
19. **ATT prompt** (Duet). On 09-05 you accepted the correction that Apple requires ATT only for cross-app tracking. No ATT prompt; the standard disclosures stay.
20. **Who gets History tools.** They go with the job: Luna for background work (09-23), Sol for conversation, consistent with "additive and guiding" (10-23).
21. **Hosting order.** The 10-22 order you agreed to (hosted access first) is the one you acted on. Implementation order is Tyler's.
22. **Clinical library scope.** It's a tag-defined PMC set, including reviews and case reports (09-13), "as long as we document the way that we're collecting it".
23. **Fixed caps on model calls and workers** (09-15, "for now"). These are retired by "The model should not be restricted outside of that" (10-23) and S5. Any remaining counts are runtime budgets, which are Tyler's call.
24. **"Couldn't finish, Try again".** This shows on the patient page and on the work itself, not on Today. Only can't-restart work that left something unhandled waits on Today (10-23).
25. **Infrastructure failures and attention.** These agree: attention comes only when a clinical responsibility is left unhandled. The four can't-restart kinds name which ones.
26. **One For my doctor workspace.** On 10-22 you agreed it stays one workspace, organized by doctor. "Send to my practice" names which practice it goes to.
27. **Agents visible to the patient** (ENG-27). You chose the "Drafted by Coda · approved by … · confirmed by …" line twice on 10-12. This is not in the MVP.
28. **"Not committing to a care loop"** (09-17). This was about committing to one clinical episode. The generic loop in Linear isn't that.

29. **C8, initial understanding that can't restart.** It waits on Today with Done. Your words: "we should have like a warning and then a retry option or something." [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-24T16:04:41.156Z]. The spec's "Updates only" came from how a question was framed, not from you.
30. **Q8, lab range band and flag.** Neither appears in the MVP. Your later words, "flags, references probably should not be there" (09-23), and "records are just like a data dump" for now (10-28), settle it.
31. **Dispatcher priority at capacity (was L4).** This is a runtime budget and Tyler's call, not product intent (PRINCIPLES.md, "Testing a rule").

---

**Changed over time.** The sweep also found 163 places where you replaced an earlier call. In each one the specs now follow your newer words, and HISTORY.md keeps the earlier call where it matters.

**Stale sources.** Linear tickets, docs and site lines that lag your words are corrected at their source once you approve the exact text.
