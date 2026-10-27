# Build requirements: unit-2332 (Peter Bowen, MRN00002332)

Source: synthetic_hospital patient 2332, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its unfinished text.

## Who he is (S)

Peter Bowen, born 1948-08-10, male, White, not Hispanic or Latino, English, Medicare,
lives in Amherst (01002), phone +1-603-555-0164. Retired high school mathematics
teacher. Former smoker, quit about 20 years ago (around 2006), 30 pack-years.
Alcohol: occasional, 1 to 2 glasses of wine a month. No known drug allergies.

Surgical history (years re-dated, R1): left inguinal hernia repair 2020; cataract
extraction with intraocular lens, right eye spring 2024 and left eye fall 2024.

Family history: father with Alzheimer disease from age 78, died at 82; mother with
hypertension, stroke at 80; one older brother with type 2 diabetes.

A daughter goes with him to visits from late 2024 and gives much of the history
(S: the neurology notes' informant). Widowed, lives in his own house (A5).

Joined the practice in August 2022 (A1: new-patient visit 2022-08-23), age 74. Brought
on joining: essential hypertension, hyperlipidemia, osteoarthritis of both knees, and a
history of cerebral small vessel disease (known from earlier care, no images received).
Drugs on joining, all continued unchanged to today: amlodipine 5 mg daily, atorvastatin
20 mg daily, aspirin 81 mg daily, acetaminophen 500 mg as needed for knee pain.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-23 | practice | new-patient visit (A1): history as above; BP controlled on amlodipine; baseline labs; prior-practice records requested; AAA screening ultrasound ordered (P) |
| 2024-05 | outside | right cataract extraction with IOL (R1); practice knows it from the patient's account |
| 2024-10 | outside | left cataract extraction with IOL (R1); patient's account |
| 2024-11-19 | practice | referral visit (A2): daughter with him; walking has slowed over months, recent urinary leakage, forgetful for a few months; slow short-stepped gait, otherwise nonfocal exam; CBC, CMP, TSH, vitamin B12 and urinalysis normal (A2); **neurology referral** |
| 2024-12-16 | outside | neurology consult (age 76): progressive walking difficulty, recent urinary incontinence, daughter reports forgetfulness over the past few months. Short shuffling steps, wide-based magnetic gait; alert and oriented with impaired recall and trouble with executive tasks. MRI brain: ventricles enlarged out of proportion to cortical atrophy, sulci normal in size, no mass or obstruction. **Normal pressure hydrocephalus diagnosed.** Reaches the practice as a short consult letter (sparse); no MRI report is received. No drug change |
| 2026-05-19 | practice (portal) | daughter writes that he has fallen more than once and is more forgetful, with more accidents before reaching the bathroom; neurology follow-up already booked (P) |
| 2026-06-12 | outside | neurology follow-up (age 77), first since December 2024 (R2): over the past 3 months memory worse (misplaces things, repeats questions), multiple falls, urge incontinence before reaching the bathroom; no headache, nausea or recent illness. Short hesitant steps, needs help to keep balance, mild cognitive impairment, impaired tandem gait, no focal deficits. Plan in the letter (A3): referral to neurosurgery for a high-volume lumbar tap test and shunt evaluation; physical therapy for gait and balance. Short letter received 2026-06-19 |
| 2026-06-26 | practice | last seen. Follow-up after neurology with daughter: falls without injury; exam as neurology found (hesitant short steps, needs a hand for balance); **history of falling** added (S: multiple falls); **outpatient physical therapy referral** placed (A4); home safety talk; drugs unchanged |
| 2026-07 to 08 | practice (portal, pharmacy) | daughter reports PT under way (A4 closes) and that a neurosurgery visit is set for the fall; refills. Nothing reconciled |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile's surgical years (hernia 2015, cataracts 2019 and 2020) are
  source-text years. They follow the re-dated timeline: hernia repair 2020 (before he
  joined), right cataract May 2024, left cataract October 2024, both before the first
  neurology visit, as the source lists them at that visit. The practice knows these
  only from the patient's account (sparse records).
- **R2.** The source's "assessed in January 2020" is the December 2024 neurology visit.
  The pronoun "she" in the 2026 history is the daughter.
- **R3.** The source lists normal pressure hydrocephalus with a diagnosis stamp; it
  is diagnosed at the 2024-12-16 neurology visit and enters the practice's problem
  list when that letter arrives. It is not known at joining.
- **R4.** The 2026 exam's "mild cognitive impairment" is an exam finding within the
  hydrocephalus, not a separate problem; no separate diagnosis is listed.
- **Left as the source has it:** between December 2024 and June 2026 there is no
  neurology contact, no lumbar tap test, no neurosurgery referral and no treatment for
  the hydrocephalus; no note comments on it. Atorvastatin stays 20 mg throughout. No
  scored cognitive test is done at the practice after the diagnosis.

## Additions (A): physician to confirm

- **A1.** 2022-08-23 new-patient visit (four years with the practice per the seed; the
  source starts at a specialist with an established medication list).
- **A2.** 2024-11-19 practice visit that sends him to neurology, with a reversible-cause
  screen (CBC, CMP, TSH about 1.8, B12 about 540, urinalysis), all normal.
- **A3.** The June 2026 neurology plan: neurosurgery referral for a high-volume lumbar
  tap test and shunt evaluation, and physical therapy. The usual next step for
  progressing hydrocephalus; nothing is done yet by today (neurosurgery visit pending).
- **A4.** Outpatient physical therapy referral from the practice on 2026-06-26 for gait
  and fall prevention, started in July per the daughter.
- **A5.** Marital status (widowed) and living in his own home: the chart needs a marital
  status and the daughter, not a spouse, is the informant. No other family member.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Levels: height 175 cm, weight about 80 kg and stable; BP on amlodipine about 132/78;
  LDL about 88, HDL about 52, triglycerides about 120 on atorvastatin 20; creatinine,
  glucose, blood counts normal for age.
- Visits for blood pressure and lipids about every 6 months; Medicare annual wellness
  visits in August 2023, 2024 and 2025 (PHQ-2 of 0 each year; before December 2024 the
  physician notes no memory concern); CMP and lipids yearly, CBC at joining and at
  wellness visits.
- Prior-practice records (received September 2022) mention a normal screening
  colonoscopy about 2016 and the 2020 hernia repair; no further colorectal screening
  at his age.
- One AAA screening ultrasound (male, former smoker, age 74): normal aortic caliber.
  Hepatitis C antibody once, negative. No lung cancer screening (quit more than 15
  years ago).
- Vaccines: influenza (high-dose) each fall, PCV20 in 2022, Tdap 2022, Shingrix two
  doses 2023, COVID each fall, RSV once in 2024.
- Knee osteoarthritis: acetaminophen as needed, walking; no imaging or injections.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit, records requests.
- No visit is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002332 on 2026-06-26. Practice contacts after that day are
only messages and refills; outside care after it still happens, and its records are
filed as received without changing the practice's lists. No visit is booked.
