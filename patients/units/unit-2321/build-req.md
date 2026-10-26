# Build requirements: unit-2321 (George Thayer, MRN00002321)

Source: synthetic_hospital patient 2321, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, its "findings are shown below" image text or its
unfinished quiz assessment.

## Who he is (S)

George Thayer, born 1953-09-20, male, White, not Hispanic or Latino, English, Medicare,
lives in Sunderland MA 01375, phone +1-508-555-0140, george.thayer@example.test.
Retired high school mathematics teacher. Never smoker. No alcohol (lifelong abstainer).
Married; his wife and a daughter come to visits (S: the source has each bring him once).

- Allergies: sulfa drugs (rash); latex (reaction not recorded).
- Surgical history: cholecystectomy 2002; transurethral resection of the prostate 2015;
  bilateral cataract extraction 2019.
- Family history: father Parkinson disease (onset at 72); mother Alzheimer disease
  (onset at 75); one older brother with essential tremor.

**Joined the practice in September 2023** (seed: about 3 years with the practice;
establish-care visit 2023-09-12, age 69, A1). He brought: hypertension on amlodipine
5 mg daily; benign prostatic hyperplasia, treated by TURP in 2015 and on no drug;
chronic constipation on docusate 100 mg twice daily and polyethylene glycol 3350 17 g
daily as needed; hyposmia (long-standing reduced sense of smell). No Parkinson disease
yet, no Parkinson drugs (R1).

Records: complete. Every outside letter and report reaches the practice within a few
days and is filed; the practice lists are kept current.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | establish care (A1): history as above; exam unremarkable apart from what the history gives; no tremor or gait change noted |
| 2024-03-19 | practice | HTN follow-up; daughter with him reports months of a resting tremor of the right hand, slower shuffling walk and moments where his feet stick mid-step; PCP sees resting tremor and slowed movement; **neurology referral placed** (A2); TSH, B12 and CMP drawn, normal (P) |
| 2024-07-29 | outside | neurology new-patient consult (letter received 2024-08-01), brought by his daughter: about a year of tremor and walking trouble; slow shuffling gait with freezing mid-step; reduced facial expression; soft monotone speech; tremor worst at rest and easing with movement; independent in daily activities but gait increasingly unsteady. Exam: bradykinesia, cogwheel rigidity, stooped posture; no sensory loss; reflexes normal; no signs of stroke. **Parkinson disease diagnosed** (R1); **carbidopa-levodopa 25/100 mg three times daily started** (R1); MRI brain ordered (R4) |
| 2024-08-20 | outside | MRI brain without contrast at an outside imaging center (R4): age-appropriate atrophy, no focal lesions; report received 2024-08-22 |
| 2025-01-21 | outside | neurology follow-up (A3, letter received 2025-01-24): gait and slowness only partly better on carbidopa-levodopa; **ropinirole started**, titrated over several weeks to **3 mg three times daily**; carbidopa-levodopa continued |
| 2026-08-11 | outside | neurology follow-up (letter received 2026-08-13), age 72, brought by his wife, about two years after the first neurology visit: over the past 3 years progressive stiffness of the limbs, slowed movement and shuffling, now with growing difficulty with daily activities; reports reduced sense of smell, constipation and trouble falling asleep; wife says his face is often blank and he sometimes takes several seconds to answer; no recent head trauma. Visual hallucinations with insight (A4). Exam: resting tremor of the right hand that eases with voluntary movement; cogwheel rigidity of both arms; bradykinesia; stooped gait with reduced arm swing; slowed processing on cognitive testing with orientation intact. MRI reviewed (age-appropriate atrophy, no focal lesion). **Parkinson disease psychosis diagnosed** (R3); **quetiapine 25 mg at bedtime started** (R3); carbidopa-levodopa 25/100 three times daily and ropinirole 3 mg three times daily continued; neurology follow-up planned (R6) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists Parkinson disease and carbidopa-levodopa from the first
  encounter, yet encounter 0 is the initial neurology evaluation of a year of tremor
  and gait trouble (encounter 1 calls it his "initial neurology evaluation").
  Resolution: symptoms begin about mid-2023 (onset 2023-07), after he joined; Parkinson
  disease is diagnosed and carbidopa-levodopa 25/100 three times daily is started at the
  2024-07-29 consult. Practice notes before that date never name either.
- **R2.** Ropinirole 3 mg three times daily is on the encounter 0 list, but a new patient
  is not started on levodopa and a full-dose agonist together. Resolution: ropinirole
  starts at the January 2025 neurology follow-up (A3). The 2024-07-29 letter lists only
  amlodipine, docusate and polyethylene glycol as home medications.
- **R3.** Quetiapine is on the encounter 0 list and the encounter 1 problem list stamps
  "Parkinson disease with psychosis" to the encounter 0 date, but encounter 0 describes
  no psychotic symptom. Resolution: Parkinson disease psychosis is diagnosed at the
  2026-08-11 visit, where quetiapine 25 mg at bedtime starts (the source's primary
  diagnosis, resting on A4). The 2026-08-11 letter lists quetiapine as newly started,
  not as a home medication.
- **R4.** Encounter 0 says an MRI "is performed" with findings "shown below"; encounter 1
  gives the result. Resolution: one MRI brain without contrast, ordered at the 2024-07-29
  consult and done 2024-08-20, showing age-appropriate atrophy without focal lesions.
  Encounter 1 refers back to it.
- **R5.** Encounter 1 says he denies any family history of similar symptoms, against the
  profile's father with Parkinson disease and brother with essential tremor.
  Resolution: the profile family history is carried everywhere; the denial is not
  carried.
- **R6.** Encounter 1's assessment stops mid-sentence in quiz text. Resolution: the
  letter's assessment is progressing Parkinson disease, now with psychosis (visual
  hallucinations with insight) and sleep-onset difficulty; plan: start quetiapine 25 mg
  at bedtime, continue the other drugs, neurology follow-up in about 3 months. No quiz
  text is carried.
- **R7.** Encounter 0 says "over the past year" and encounter 1 "over the past 3 years".
  Both fit onset about mid-2023 (R1); kept.
- **Left as the source has it:** ropinirole stays at 3 mg three times daily after
  psychosis is diagnosed; no one lowers it and no note comments on it. No physical,
  occupational or speech therapy referral is made for the gait and voice changes. No
  formal neuropsychological testing is ordered for the slowed processing.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 establish-care visit (seed: about 3 years with the practice).
- **A2.** The 2024-03-19 practice visit that refers him to neurology.
- **A3.** The 2025-01-21 neurology follow-up that starts ropinirole (the source lists it,
  so it must start somewhere; R2).
- **A4.** At 2026-08-11 his wife describes about two months of seeing people in the
  house who are not there, mostly in the evening; he recognizes they are not real; no
  delusions. This is the evidence for the source's Parkinson disease psychosis.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from his prior primary care practice (received September 2023) may mention
  routine screening done there with normal results: a screening colonoscopy in 2018
  without polyps; Tdap in 2019; the surgeries above.
- Blood pressure follow-up every 6 months or so on amlodipine 5 mg, controlled (about
  130/78). Weight about 74 kg, height 175 cm, steady.
- Annual wellness visits (initial 2024, subsequent 2025) with CMP, CBC, lipids, A1c,
  PHQ-2 and a cognitive screen, all normal. Hepatitis C and HIV screening once, at
  establishment, nonreactive.
- Vaccines: PCV20 at establishment; influenza (high-dose) and COVID each fall; Shingrix
  two doses 2023-2024; RSV once in 2024.
- Refills of amlodipine, docusate and polyethylene glycol from the practice; neurology
  prescribes the Parkinson drugs and quetiapine. Pharmacy and portal traffic, a
  cancelled visit, calls from his wife or daughter about scheduling.
- After the 2026-08-11 letter: the PCP reviews it and the practice calls his wife about
  the new medication; nothing else is open.
- Booked visit after 2026-08-27: a 2026-09-22 office visit (wellness visit and follow-up
  after the neurology change).

No acute illnesses, injuries, falls, new diagnoses or life events beyond the timeline.
