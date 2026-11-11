# Open decisions

Everything here is undecided. The specs hold only what's decided, so when one of these is answered, the answer goes into every spec it touches and the question leaves this list. Answer by ID, for example "Q5: they see a short notice".

## Product decisions

**Physician**

- **Q2. Practice access and roles.** Which patients each physician may access, and whether a practice has roles. Patient messaging waits on it for who sees a message before it's matched, who sees a patient's phone and email while matching, and who may send.
- **Q3. Judging necessary care in the simulation.** How the patients-who-do-nothing run judges that necessary care happened, and who judges it.

**Duet**

- **Q4. You while AI isn't allowed.** Whether the person can add or edit notes and medications in You while AI isn't allowed.
- **Q5. An interrupted reply.** What the person sees while an interrupted reply recovers. The unfinished draft is replaced, as the [harness](harness.md#guarantees) promises.
- **Q6. Deleting an email or Google account.** How a person who signs in with email or Google reconfirms before deleting their account.
- **Q7. Newer Terms.** How the app shows newer Terms and takes acceptance.

**Connected care**

- **Q8. An expired or cancelled invite.** What the patient sees when they open one.
- **Q9. An invite opened but not shared.** What happens to the invite, and what the practice sees, when a patient opens it and doesn't tap Share.
- **Q10. Leaving a practice.** Whether Duet confirms before the patient leaves, and what it says.
- **Q11. Work in progress when a connection ends.** What happens to work the practice's agents are doing on that patient's Duet data.
- **Q12. The discharge notice.** What it says, such as the reason and who to contact.
- **Q13. After a rejoin.** Whether the practice sees the disconnected period marked.
- **Q14. Declining invites.** Whether a patient can decline future invites from a practice.
- **Q15. Account deletion copy.** The confirmation wording when a connected patient deletes their account.

**UI** (judged against PRINCIPLES.md U1–U9)

- **UI1. Duet's Today.** The order and density of insights, appointments, Plans, Check-ins, the doctor's plan, For my doctor and measurements, and whether another overview adds anything.
- **UI2. Entry and return.** Where the controls that move between Today and chat sit.
- **UI3. Saved forms.** Which outputs beyond Plans, Check-ins and For my doctor need a saved, editable form.
- **UI4. Uncertain appointment dates.** How they look. Duet judges when a reminder about one is useful.
- **UI6. Deleting a physician conversation.** Whether Delete sits beside Archive.
- **UI7. The Updates log.** Whether the full log keeps its own page beyond Today's collapsed line, and how it looks.
- **UI8. Duet's companion.** Whether the companion stays, whether the chat header shows its name, and whether its one-time relationship moments are wanted. If it stays, its name stays on the face and the chat stays Duet.

**After the MVP**

- **F1. Coda's own clinical record.** Whether Coda keeps its own record or builds into EHRs. In the MVP the EHR is the source of truth, read only, and Duet data a practice receives lives in Coda.
- **F2. The patient as co-owner of the record.**
- **F3. Calendar and email connections in Duet.**
- **F4. NPI registry ingestion** as its own groundwork.
- **F5. Pricing.** Duet stays free until pricing is chosen. Whatever premium becomes, it only includes what exists because the patient has a doctor on Coda.
- **F6. A Hint route** that doesn't depend on the practice's open Chrome.
- **F7. Background-run limits.** There's no limit in the MVP. A later limit might cap runs per patient by what the practice pays.
- **F8. Reference ranges on For my doctor,** once their sourcing is better.

**Studies**

- **S1. Regexes.** List every product regex that infers meaning from free text or model output, with keep or remove for each.
- **S2. CDS compliance.** Whether a lightweight deterministic contract should back the prompts.

**Before production**

- **B1. Epic questionnaire and privacy policy.** Correct the draft Epic Data Use Questionnaire and the privacy wording to match what Duet does when a source is disconnected.

## Engineering decisions

- **E1. Production triggers and the schedule source.** How scheduled and pre-visit work runs in production, and where the real schedule comes from.
- **E2. Schedule mapping.** How each EHR's schedule and encounters map to patients.
- **E3. The patient roster.** How the first practice populates and maintains its active roster remains open. Decide after checking what its sources provide and what the practice needs.
- **E4. Freshness after a one-time pull.** How records from other systems stay fresh.
- **E5. athena.** The patient-access route and what's still missing.
- **E6. Push registration.** Fixing the push problem from the security review, now that notifications follow whoever is signed in.
- **E7. Chart search.** Outline first or plain search, settled by evals.
- **E8. Browser work.** Duet's own browser loop as primary with native computer use as fallback is the leaning, not a decision.
- **E9. The Anthropic BAA.** It exists, but nothing uses it for product calls yet; product calls go through the OpenAI API.
- **E10. Prompt injection.** A capability lane with no design yet.
- **E11. The simulation's clock.** What "today" means once the clock moves.
- **E12. Run outcomes and events.** The run outcome vocabulary (finished, interrupted, failed, cancelled), and whether runs emit run-started and checkpoint-written events.
- **E13. Physician run preconditions.** What the physician product checks before each run, stated in harness terms.
- **E14. Duet export.** The export format and how it's delivered.
- **E15. Loading tools and skills.** Which tools and skills the model gets up front and which it finds on request. Dogfooding informs it.
- **E16. Earlier tool results after a fresh window.** Whether the model can list them, or must read the source again.
- **E17. Recent messages in a fresh window.** Whether a fresh window also carries the last few messages word for word.
- **E18. Pausing for the person.** Whether a run can stop to ask the person a question, or for approval of an exact action, and resume with the answer, as Codex, Hermes and OpenClaw can. Closing it means deciding how the model asks (a tool), how a waiting run is kept (held durably, taking no slot), how the answer returns (a saved input carried into the next request once, like a steer), what happens when no answer comes, and that an approval covers exactly the action shown. Physician write-back needs it first.
- **E19. Steering a running worker.** Whether the lead can message or stop a worker while it runs, as Codex and OpenClaw allow.
