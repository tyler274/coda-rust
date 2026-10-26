# Build requirements: unit-1917 (Tracy Leblanc, MRN00001917)

Source: synthetic_hospital patient 1917, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a hole;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its second-person exam text ("you note that").

## Who she is (S)

Tracy Leblanc, born 1989-02-18, female, White, not Hispanic or Latino, English,
Chesterfield MA 01012, Blue Cross Blue Shield commercial plan. Elementary school
teacher. Never smoker. Alcohol socially, 1 to 2 drinks a month. No drug use.
Allergies: no known drug allergies. Surgical history: wisdom teeth extraction at
age 22 (about 2011). Family history: mother with type 2 diabetes, father with
hypertension, sister with a past Bell palsy. No chronic conditions, no regular
medications.

Joined the practice in August 2022 (A1), age 33, healthy, bringing no problems and
no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-16 | practice | new-patient visit (A1): history as above, normal exam, prevention brought up to date (P) |
| 2025-01-30 | outside, ED | woke that morning unable to close the right eye or smile properly; no recent illness, trauma or other neurological symptoms. Exam: cannot wrinkle the right forehead, close the right eye or show teeth on the right; other cranial nerves normal; no vesicles in the external auditory canal; hearing intact; rest of neuro exam normal. **Bell palsy, right, diagnosed.** **Prednisone 60 mg daily for 7 days** and **lubricating eye drops** with eye taping at night (A2). Told to follow up with her PCP. The practice receives the ED record the next day |
| 2025-02-04 | practice | post-ED visit (P): day 6 of prednisone; right facial weakness still present, eye care reviewed, school note. Recovery expected; no referral |
| 2025-04-22 | practice, portal | patient reports her face is back to normal (A3); Bell palsy marked resolved |
| 2025-09-22 | practice, phone | since the morning of 2025-09-21: dribbling liquids from the left corner of the mouth, food tasting dull at the front of the tongue (R2). Other side this time, so the PCP arranges an urgent neurology visit for the next day (A4: the referral) |
| 2025-09-23 | outside, neurology | about 8 months after the right-sided episode in January 2025 (R1): trouble drinking without dribbling from the left mouth corner; food tastes dull at the front of the tongue, normal at the back; no headache, fever or recent illness. Exam: reduced taste on the left anterior tongue, incomplete left eyelid closure, mild left smile asymmetry; facial sensation normal, tongue midline, jaw strength normal. **Bell palsy, left (recurrence, opposite side), diagnosed.** **Prednisone 60 mg daily for 7 days** and lubricating eye drops (A5). No labs or imaging (left as the source has it). Letter received 2025-09-26 |
| 2025-09-24 | practice | **last seen.** Annual physical, already scheduled (P): day 2 of prednisone, left facial weakness as the neurologist found it, eye care reviewed. Prevention, flu and COVID vaccines |
| 2026-05-25 | outside, neurology | 2 days of progressively worsening right facial weakness (onset 2026-05-23); woke with a crooked smile and trouble closing the right eye; follows episodes in January and September 2025 (R1); no recent illness, trauma or other neurological symptoms; takes no medications. Exam: cannot wrinkle the right forehead, close the right eye or smile on the right; taste reduced on the right anterior two-thirds of the tongue; other cranial nerves normal; otoscopy normal; no rash or vesicles at the ear or in the mouth; no other deficits; rest of exam normal. Labs: CBC normal, ESR 15 mm/hr, glucose 90 mg/dL, Lyme serology negative. **Recurrent Bell palsy, right.** **Prednisone 60 mg daily for 7 days** and lubricating eye drops (A5); MRI ordered (A6). Letter received 2026-05-28; practice lists unchanged (after last seen) |
| 2026-06-09 | outside, imaging | MRI brain with and without gadolinium, including the internal auditory canals (A6): normal; no enhancing lesion along either facial nerve, no mass. Report received 2026-06-11 |
| 2026-08-11 | outside, neurology | follow-up (A7): right face much improved, eye closes fully, slight residual right smile asymmetry; MRI reviewed as normal; no further testing; return if another episode. Letter received 2026-08-13 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates the episodes "January 2020" and "September 2020". Resolution:
  they follow the re-dated timeline, January 2025 and September 2025. "Eight months
  after" the January episode fits 2025-01-30 to 2025-09-23.
- **R2.** The September 2025 consult gives no onset for the left-sided symptoms.
  Resolution: they began on the morning of 2025-09-21, two days before the consult,
  so the episode is acute and treated as one (A5).
- **R3.** The source's encounter notes copy "Bell's palsy" onto an active problem list
  at every visit. Resolution: the practice list follows the episodes: right Bell palsy
  diagnosed 2025-01-30, resolved 2025-04-22; diagnosed again (left) 2025-09-23 and still
  active on the practice list at the last visit. The May 2026 episode appears only in the
  neurology letter (after last seen); the practice list is not updated.
- **R4.** The source's 2026 labs say only "CBC within normal limits". Resolution: a CBC
  with ordinary normal values; the letter states the CBC as normal.
- **Left as the source has it:** no labs or imaging at the second (opposite-side)
  episode in September 2025, and no workup for the sister's history. No note remarks
  on it.

## Additions (A): physician to confirm

- **A1.** The 2022-08-16 new-patient visit (the seed gives about four years with the
  practice; the source's first encounter is an ED visit with no practice yet named).
- **A2.** Prednisone 60 mg daily for 7 days, plus lubricating eye drops and taping the
  eye shut at night, at the January 2025 ED visit: standard care for Bell palsy seen
  within 72 hours, and the source gives no plan.
- **A3.** Full recovery of the right-sided episode by April 2025, needed so the
  September episode can be a new one on the other side ("returns ... after a prior
  episode").
- **A4.** The 2025-09-22 phone call that sends her to neurology.
- **A5.** The same prednisone course and eye drops at the September 2025 and May 2026
  neurology visits (each seen within 72 hours of onset). By the May visit she "takes no
  medications": each course had finished.
- **A6.** MRI brain with and without contrast through the internal auditory canals
  after the third episode, normal: the usual next step for recurrent, alternating
  facial palsy.
- **A7.** The August 2026 neurology follow-up: improving right face, normal MRI, return
  as needed.
- **A8.** The left-sided episode improving by November 2025 (eye closes, taste mostly back, smile asymmetry only in photos), told by the patient in a message after the last visit; recovered by the May 2026 neurology visit, as that letter records.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior primary-care office (received August 2022): no problems, normal
  cervical cytology with negative HPV in 2019, Tdap 2017, COVID primary series and booster
  2021.
- Height about 165 cm; weight about 63 kg, steady (BMI about 23). Blood pressure normal.
- Yearly physicals (2023-08-15, 2024-09-10 after a cancelled 2024-08-27 date,
  2025-09-24), with PHQ-2 or PHQ-9 screens (normal). Labs: CBC, CMP, lipids, HIV and
  hepatitis C screening once at the new-patient visit; CBC and CMP 2023; CMP, lipids and
  A1c 2024 (family history of diabetes); all normal.
- Cervical cytology with HPV co-test at the 2024 physical: negative.
- Influenza vaccine each fall (practice or pharmacy), COVID vaccines (bivalent 2022,
  updated formula 2023, 2024, 2025). Tdap not due before today.
- Portal messages about results, school forms and immunization records, scheduling,
  membership admin; ED and neurology correspondence.
- No booked visit after 2026-08-27; the practice has not seen her since 2025-09-24.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001917 on 2025-09-24. Practice contacts after that day are
messages only; outside care after it still happens, and its records are filed as
received without changing the practice's lists. No visit is booked after today.
