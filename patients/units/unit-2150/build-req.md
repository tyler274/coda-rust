# Build requirements: unit-2150 (Douglas Doyle, MRN00002150)

Source: synthetic_hospital patient 2150, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished text.

## Who he is (S)

Born 1968-10-08, male, White, not Hispanic or Latino, English, lives in South Hadley,
commercial PPO through his employer. Warehouse loader, 30+ years of heavy manual labor.
Former smoker, 20 pack-years, quit in 2019 ("quit 5 years ago" as of July 2024).
Alcohol occasional, 1 to 2 beers on weekends.

Allergies: sulfa medications (urticaria); shellfish (reaction type unknown).

Surgical history (years re-dated with the timeline, R1): left rotator cuff repair 2016;
lumbar laminectomy L4-L5 2022 (spring, at an outside spine surgery group, before he
joined).

Family history: father with cervical spondylosis with myelopathy and hypertension;
mother with osteoarthritis and osteoporosis; older brother with cervical radiculopathy,
status post anterior cervical discectomy and fusion.

Joined the practice 2022-09-14 (new patient visit, A1) from a prior primary-care
office. He brought: cervical spondylosis with multilevel degenerative disc disease
(C5-C6, C6-C7), hypertension, mechanical low back pain (after the laminectomy),
bilateral knee osteoarthritis; lisinopril 10 mg daily, ibuprofen 600 mg three times
daily as needed for pain, cyclobenzaprine 5 mg at bedtime as needed for muscle spasm,
acetaminophen 500 mg as needed for headache. Cervical radiculopathy is not known at
joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-14 | practice | new patient visit (A1): history as above; BP on lisinopril near goal; Tdap; labs; low-dose CT lung screening ordered (A2) |
| 2022-09-28 | outside | prior PCP records received: the problems and drugs above, the 2022 laminectomy, a normal screening colonoscopy in 2019 (P) |
| 2022-11-09 | outside | low-dose CT lung screening: Lung-RADS 1 (A2) |
| 2024-06-17 | practice | about 2 weeks of left neck pain into the left arm with numbness of the thumb and index finger, no trauma; mild weakness of left wrist extension on exam; **MRI cervical spine ordered; PM&R referral** (A3) |
| 2024-06-26 | outside | MRI cervical spine (S: "herniated disc at the C6-C7 level with nerve root compression"): multilevel degenerative disc disease at C5-C6 and C6-C7; left-sided disc herniation at C6-C7 compressing the left exiting nerve root; no cord compression (R2) |
| 2024-07-03 | outside | PM&R consult letter: 4 weeks of worsening neck pain radiating down the left arm with numbness of the thumb and index finger; no trauma; long manual labor. Exam: weakness of left biceps and wrist extensors; reduced sensation over the lateral forearm and first two digits; left brachioradialis reflex diminished. MRI as above. **Cervical radiculopathy (left) diagnosed** (R2). Plan: physical therapy, keep ibuprofen and cyclobenzaprine as needed, avoid heavy overhead lifting for now, PM&R follow-up in 6 to 8 weeks (R3) |
| 2024-07-10 to 2024-09-04 | outside | physical therapy course for the neck (A4): evaluation and discharge summary received |
| 2024-08-21 | outside | PM&R follow-up letter: left arm numbness and weakness largely resolved, strength now full, home exercise program, return as needed (A4) |
| 2024-09-25 | practice | physical: left arm symptoms resolved; **cervical radiculopathy resolved** on the list (A4) |
| 2025-12-28 | outside | ED visit (Sunday): weakness and numbness of the right arm for two weeks after lifting heavy boxes overhead at work, with trouble gripping; weakness worse gripping tightly or pushing open doors; tingling from the shoulder down the back of the arm and forearm into the middle finger; worse tilting the head right or looking up for long periods. Exam: right triceps reflex decreased compared with the left; reduced strength of wrist extension and elbow extension, shoulder abduction and finger abduction preserved; diminished pinprick over the posterior forearm and dorsal third digit; Spurling's maneuver reproduces the symptoms. Cervical spine x-ray: degenerative changes with disc space narrowing and osteophyte formation. Assessment: compression of the right C7 nerve root, **cervical radiculopathy (right) diagnosed again** (R4). Discharged home on his own medicines, avoid overhead lifting, primary-care follow-up within a week for outpatient MRI (R4) |
| 2026-01-05 | practice | post-ED visit (A5): findings as in the ED, improving slightly; **MRI cervical spine ordered; PM&R referral**; work note: no overhead lifting and no lifting over 20 lb for 4 weeks |
| 2026-01-12 | outside | MRI cervical spine (A5): known degenerative disc disease at C5-C6 and C6-C7; right foraminal disc herniation at C6-C7 compressing the right exiting C7 root; left C6-C7 foramen patent; no cord compression |
| 2026-01-21 | outside | PM&R consult letter (A5): right C7 radiculopathy; physical therapy, light duty, ibuprofen and cyclobenzaprine as needed; if not better in 6 to 8 weeks, consider a cervical epidural steroid injection or a spine surgery opinion |
| 2026-02-24 | practice | last visit (last_seen): right arm tingling much better, grip improving, in PT, back at work on light duty; BP at goal; BMP drawn |
| 2026-04-08 | outside | PM&R follow-up letter, filed as received (A5): right arm strength full, tingling resolved, no injection needed, home exercise program, return as needed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Source text years follow the re-dated timeline (the source shifts by about
  4.5 years): the 2018 laminectomy becomes 2022 and the 2012 rotator cuff repair 2016;
  the ED note's "January 2020" PM&R visit is July 2024; "quit 5 years ago" in 2024
  means he quit in 2019.
- **R2.** The PM&R exam describes a C6 pattern (biceps, wrist extensors, lateral
  forearm, thumb and index, brachioradialis reflex) while the MRI names the C6-C7
  disc. The letter carries both as written and names the diagnosis left cervical
  radiculopathy without assigning a single root. No note remarks on the mismatch.
  The source profile lists cervical radiculopathy as a chronic condition from the
  start; the chart diagnoses it on 2024-07-03.
- **R3.** The PM&R note gives no plan. The letter's plan is conservative: physical
  therapy and his existing as-needed medicines. No new drug.
- **R4.** The ED note says he "returns to the emergency department", but no earlier ED
  visit exists; the word maps to the recurrence of arm symptoms. Its plan stops at
  "Compression of": the assessment is compression of the right C7 nerve root, and the
  plan is discharge on home medicines with primary-care follow-up. No ED medication is
  given. The ED's copied problem list carries cervical radiculopathy as active; the
  practice list had it resolved in September 2024, and the ED visit diagnoses it again
  on the right.
- **Left as the source has it:** ibuprofen 600 mg up to three times daily continues
  alongside lisinopril and hypertension through the whole chart; no one moves him off
  it and no note comments on it. Lisinopril stays 10 mg throughout.

## Additions (A): physician to confirm

- **A1.** The 2022 new-patient visit (the source's first encounter already lists his
  chronic problems, so he was a patient before July 2024; the seed gives 4 years with
  the practice).
- **A2.** Low-dose CT lung cancer screening yearly (20 pack-years, quit 2019): 2022-11-09,
  2023-11-08, 2024-11-13, 2025-11-12, each Lung-RADS 1, no nodules.
- **A3.** The 2024-06-17 practice visit that orders the MRI the PM&R letter reviews and
  makes the referral.
- **A4.** The usual course after the 2024 PM&R visit: physical therapy (evaluation
  2024-07-10, discharge 2024-09-04), the 2024-08-21 PM&R follow-up with full strength,
  and the problem resolved on the list at the 2024-09-25 physical.
- **A5.** The usual course after the 2025 ED visit: the post-ED practice visit, the
  2026-01-12 MRI with the findings given in the timeline, the 2026-01-21 PM&R consult,
  physical therapy from late January 2026, and the 2026-04-08 PM&R follow-up (after
  last_seen, filed as received without changing the practice's lists).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (alignment, no fracture, cord signal, lungs on CT). An abnormal finding appears only
  as the source or this file gives it. A report may carry the descriptors its kind
  always carries, but nothing that adds a territory, a severity grade or a symptom, or
  changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (the 2019 colonoscopy, no polyps, repeat in 10 years).
- Hypertension visits about every 6 months and a yearly physical, with vitals; CMP or
  BMP, CBC, lipids and A1c on ordinary normal values; blood pressure controlled on
  lisinopril 10 mg (about 130s/80s). Knee and low back pain mentioned as stable at
  physicals, managed with his as-needed medicines.
- Prevention: Tdap at joining; Shingrix two doses in 2023; influenza each fall; updated
  COVID vaccines each fall (pharmacy); hepatitis C and HIV screening once (negative);
  PHQ-2 at physicals (negative); the 2019 colonoscopy covers colorectal screening.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed visit (2025-03-11, rebooked), reminders, the work note.
- No visit is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002150 on 2026-02-24. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. No visit is booked after today.
