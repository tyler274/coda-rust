# Decisions for Dev

One place for every open question across physician.md, duet.md, connected-care.md and harness.md. Settled answers live in the specs, each line citing the words that settled it; the specs carry no open questions of their own, and a step no source states is marked `GAP:` where it occurs. Every quote is verbatim from its source. Pointers follow the specs: `claude:<session>…@<UTC time>` and `codex:…<session>…@<UTC time>` are Dev's words, and `(chosen)` marks an option Dev clicked. `linear:` is Linear as read on 2026-10-28; a Linear line with no date of its own is marked "undated". `(doc)` and `(site)` are not Dev's words.

Answer by ID, for example "O8: a join starts them" or "T6: recommendation".

---

## Open for Dev

These are things no source answers yet, or places where two spec lines disagree, gathered here so there is one list. Steps no source states stay in the specs as `GAP:` lines; each is settled when that journey's chart is built, and one that needs a product choice comes here.

### Product decisions

- **O9. Priority when the rate-limit gate reopens.** harness G15 calls it product policy (physician chat first). An earlier settlement called it a runtime budget and Tyler's call, not product intent.
- **O12. Practice roles.** Who manages messaging and who may send, once practices have real roles. Until then the messaging manager is whoever connected Gmail or set up Spruce.
- **O13. Judging necessary care in the simulation.** How L1 judges that necessary care happened, and who judges it.

### Engineering decisions

- **T1. Production triggers and the schedule source.** How scheduled and pre-visit triggers run in production, and the real schedule source. The agents judge which visits to prepare and how far ahead (PRINCIPLES.md 4). [codex:…01a081e0…@2026-09-10T14:31:19; codex:…01a0602a…@2026-09-02T19:36:35] [linear:ENG-31#Discuss with Dev]
- **T2. The patient roster.** How the active roster is kept, likely from the EHR's patient list (with ENG-57). [codex:…01a0602a…@2026-09-03T14:27:48; codex:…01a0602a…@2026-09-03T14:31:55]
- **T3. Freshness after a one-time pull.** How records from other systems stay fresh. Every source shows when it was last refreshed (Freshness, decided). [codex:rollout-2026-09-02T23-10-23-01a0653f-0357-72a0-975d-7d195fe3@2026-09-03T14:25:44.920Z]
- **T4. athena.** The patient-access route and the remaining gap. [linear:ENG-162#Discuss with Dev when you get here]
- **T5. Push registration.** Fix the security review's push problem now that notifications follow whoever is signed in. [linear:ENG-34#Discuss with Dev when you get here; claude:bbe5dff2-f358-4006-98a2-9930babf8314.jsonl@2026-10-26T17:15:32.634Z]
- **T6. Chart search.** Outline first or plain search, settled by evals. [claude:220b86fb…@2026-09-24T17:09:14; claude:220b86fb…@2026-09-24T17:46:31]
- **T7. Browser work.** Luna's own browser loop as primary and native computer use as fallback; your Sept 30 leaning, not decided. [codex:…01a0efb2…@2026-09-30T02:23:52]
- **T8. Anthropic BAA.** It exists, but no source uses it for product calls; Linear names the OpenAI API. [codex:rollout-2026-10-16T23-44…@2026-10-17T16:54:06; linear:ENG-160#Decided]
- **T9. Prompt injection.** Named as a capability lane, with no design recorded. [codex:…@2026-09-19T00:52:39]
- **T11. Schedule mapping.** How each EHR's schedule and encounters map to patients.
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
- L3 (companion) below also waits for the UI program.

### After the MVP

- **A1. Coda's own clinical record.** Whether Coda keeps its own record or builds into EHRs. In the MVP the EHR is the source of truth, read only (physician section 7), and Duet data a practice receives lives in Coda (connected-care principle 2). [codex:…01a0f98d…@2026-10-09T20:21:01] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T18:12:44.566Z]
- **A2. Who may send** once Coda can send; practice roles aren't built. (doc) [coda-agents/docs/patient-messaging.md:1146]
- **A3. The patient as co-owner of the record.** Still being explored. [claude:121aaddd-b90e-4057-8cb5-1741b94d8162.jsonl@2026-10-12T17:21:24.135Z]
- **A4. Duet calendar and email connections.** [linear:ENG-169#Discuss with Dev when you get here]
- **A5. NPI registry ingestion** as separately scoped groundwork. [linear:ENG-183#Intended later experience]
- **A6. Premium and pricing.** Duet stays free until you pick pricing. The standing bound is that premium only includes what exists because the patient has a doctor on Coda. [claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T17:31:44.942Z; claude:1832669a-01e1-4365-916f-ae03e08161f4.jsonl@2026-10-20T20:42:55.700Z] [claude:07b69ca9-4860-49e2-b5ff-feea7e7e6130.jsonl@2026-10-13T18:48:21.355Z (implied)] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:31:31.977Z (chosen)]
- **A7. A Hint route** that doesn't depend on the practice's open Chrome. [linear:ENG-37#Discuss with Dev] [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:34:59.355Z] [linear:ENG-64#open-brainstorm-with-dev-before-building]
- **A8. ENG-159 boundary changes.** Changing clinical authority, sharing product state, unifying the products' context or saving policies, or exposing execution reasoning in Duet would each be a new decision. [linear:ENG-159#Discuss with Dev when you get here]
- **A9. Background-run limits.** No limit in the MVP. Later there may be a limit on total background runs, such as a cap on runs per patient by what the practice pays. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:03:35.292Z]
- **A10. Reference ranges on For my doctor.** Not in the MVP; ideally later, once their sourcing is better. [claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-29T01:38:41.451Z]

### Before production

- **BP1. Epic questionnaire and privacy policy.** Correct the draft Epic Data Use Questionnaire and the privacy wording to match Duet D2.15. [codex:rollout-2026-09-05T10-36-49-01a07200-2c17-78d0-aac6-b0f0cec0@2026-09-05T14:36:52.863Z; claude:6035d74e-78c3-41fc-bc5f-5ec2c0d6a5c1.jsonl@2026-10-28T20:29:06.679Z (chosen)]

---

## Later / ideal-state

### L3. Does the companion survive, does the chat header show its name, and are its one-time "relationship moments" (approved 10-02) wanted?
- **A. The companion names the chat.** The chat header shows the companion's name: "Show companion's name" [claude:04e60e17…@2026-09-28T03:48:15 (chosen)], 2026-09-28.
- **B. The chat is Duet's AI.** "rename Luna to Duet's AI" [claude:1832669a…@2026-10-20T21:14:08], 2026-10-20; Plans and Check-ins move "out of the companion card" [linear:ENG-34#What the person can do].
- **Specs:** duet (Companion).
- **Recommendation:** decide with the UI program. If the companion stays, its name stays on the face and the chat stays Duet's AI.

---

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
10. **The scribe in V1**. Out of the MVP (physician Scope).
11. **Context checkpoints**. Dev, 09-24: "they should always be secodnary". This is superseded by ENG-159 (2026-10-20), which keeps physician model-written window checkpoints.
12. **Daily re-pull vs configurable cadence** (physician mental models). Your 09-28 pick chose a method: full re-pull rather than vendor change feeds. On 10-26 you said "Keep cadence configurable". So full re-pull is the method, and the cadence is configurable, starting at daily. A record missing from a re-pull is noticed, never deleted on absence alone.
13. **Code deciding evidence is "unchanged"** (harness, What the harness must never do). Doc vs Linear: ENG-14's source checks "start a full review only when something changed". harness.md now scopes the doc line.
14. **The site showing share-sheet sharing**. Dev, 2026-10-22: "site can run ahead of the app".
15. **Notes the patient asks Duet to save** (connected-care vs duet). The 10-26 "Do it, then show" choice satisfies ENG-39's rule, "A note from conversation crosses only after the patient saves it". connected-care.md now covers both paths.
16. **The run baseline vs news mid-run** (harness doc vs products). ENG-160 (Dev, 2026-10-27) takes in news before a run finishes and records the exact versions read. harness.md now says so.
17. **The model setting its own later work**. On 09-04 you said yourself "I guess we never talked about that". Since then your words consistently favor it, through "anything, it's free" (10-28). The model may set its own later work, through follow-ups and Duet's own work.
18. **How running work shows** (physician UI). Settled by your 10-02 pick, "Three pulsing dots". Details wait for the UI program.
19. **ATT prompt** (Duet). On 09-05 you accepted the correction that Apple requires ATT only for cross-app tracking. No ATT prompt; the standard disclosures stay.
20. **Who gets History tools.** They go with the job: Luna for background work (09-23), Sol for conversation, consistent with "additive and guiding" (10-23).
21. **Hosting order.** The 10-22 order you agreed to (hosted access first) is the one you acted on. Implementation order is Tyler's.
22. **Clinical library scope.** It's a tag-defined PMC set, including reviews and case reports (09-13), "as long as we document the way that we're collecting it".
23. **Fixed caps on model calls and workers** (09-15, "for now"). These are retired by "The model should not be restricted outside of that" (10-23) and "anything, it's free" (10-28). Any remaining counts are runtime budgets, which are Tyler's call.
24. **"Couldn't finish, Try again".** This shows on the patient page and on the work itself, not on Today. Only can't-restart work that left something unhandled waits on Today (10-23).
25. **Infrastructure failures and attention.** These agree: attention comes only when a clinical responsibility is left unhandled. The four can't-restart kinds name which ones.
26. **One For my doctor workspace.** On 10-22 you agreed it stays one workspace, organized by doctor. "Send to my practice" names which practice it goes to.
27. **Agents visible to the patient** (ENG-27). You chose the "Drafted by Coda · approved by … · confirmed by …" line twice on 10-12. This is not in the MVP.
28. **"Not committing to a care loop"** (09-17). This was about committing to one clinical episode. The generic loop in Linear isn't that.

29. **C8, initial understanding that can't restart.** It waits on Today with Done. Your words: "we should have like a warning and then a retry option or something." [claude:220b86fb-906c-44c1-a206-2c4016c51a61.jsonl@2026-09-24T16:04:41.156Z]. The spec's "Updates only" came from how a question was framed, not from you.
30. **Q8, lab range band and flag.** Neither appears in the MVP. Your later words, "flags, references probably should not be there" (09-23), and "records are just like a data dump" for now (10-28), settle it.
