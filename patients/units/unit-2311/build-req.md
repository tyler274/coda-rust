# Build requirements: unit-2311 (Arthur Cummings, MRN00002311)

Source: synthetic_hospital patient 2311, in `source.md` beside this file. This file and
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
"(diagnosed 2023-01-15)" stamps, or its unfinished plan text.

## Who he is (S)

Arthur Cummings, born 1948-07-24, male, White, not Hispanic or Latino, English,
Medicare and Medicaid, Granby MA 01033. Retired postal worker. Former smoker, quit
2005, 25 pack-years. No alcohol since 2018 (stopped as his memory declined).
Allergies: penicillin (anaphylaxis), sulfa drugs (rash). Surgical history: bilateral
cataract extraction with lens implants (2020); open inguinal hernia repair (2016).
Family history: mother with Alzheimer dementia beginning at 79; father with coronary
artery disease and a myocardial infarction at 76; a sister with type 2 diabetes.
His daughter is his main support and comes to visits (S: she brings him to the ED).
Marital status widowed (A6; the ledger needs one, and no spouse appears anywhere).

**Joined the practice 2023-08-22** (A1, a new-patient visit; the source's first
encounter is an ED visit in which he already carries a full problem and drug list).
He lives with his daughter in Granby at joining (R3). He brings:

- Problems: Alzheimer's disease, moderate stage; essential hypertension;
  osteoarthritis of both knees; benign prostatic hyperplasia.
- Drugs: donepezil 10 mg at bedtime; memantine 10 mg twice daily; amlodipine 5 mg
  daily; tamsulosin 0.4 mg at bedtime; acetaminophen 500 mg every 6 hours as needed
  for knee pain; polyethylene glycol 3350 17 g daily as needed for constipation.
  The dementia drugs were started before joining by an outside clinician; the
  practice takes over the refills (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient visit with his daughter (A1): history and drugs as above; BP controlled on amlodipine; knee crepitus; oriented to person, needs prompting; establishing labs (P) |
| 2023-11-12 (Sun) | outside | regional hospital ED, brought by his daughter: two days of worsening confusion and agitation, wandering at night, not recognizing family members. Agitated further when held for IV placement. Disoriented to time and place, oriented to person, cannot follow complex commands, easily distracted; vital signs stable; otherwise unremarkable exam. Na 140, K 4.2, Cl 102, HCO3 24, BUN 18, Cr 1.0, glucose 98, Ca 9.2; CBC normal; urinalysis normal; non-contrast head CT without acute abnormality. **Delirium superimposed on dementia diagnosed.** No cause found; discharged home with his daughter after evaluation (A2). The practice receives a short ED note (2023-11-20) |
| 2023-11-13 | practice | phone call from the daughter about the ED visit; follow-up booked (P) |
| 2023-11-16 | practice | post-ED visit (P): calmer, sleeping better, still more muddled than usual per daughter; exam oriented to person only; plan to watch, no drug changes |
| 2023-12-14 | practice | follow-up: back to his usual baseline per daughter. **Delirium resolved** (R1) |
| 2024-02-20 | practice | dementia and BP follow-up: night wandering continues at home and the daughter cannot keep him safe overnight; placement discussed (R3) |
| 2024-03-25 | practice | telephone: the daughter reports he moved into a nursing home last week (R3); facility medication orders sent, list unchanged |
| 2025-07-14 | practice | **last held visit** (routine follow-up, P) |
| 2025-07-29 (Tue) | outside | regional hospital ED then ICU (R2: re-dated from 2025-10-27). Brought in by family during a July heatwave from his nursing home, which has no air conditioning; a day of being less responsive and more confused. Temperature 41.1 C (106 F), HR 110, BP 90/60, RR 24. Lethargic, disoriented, cannot answer questions coherently; skin hot and dry. Na 150, K 3.2, Cr 1.8, BUN 30; arterial gas pH 7.30, PaCO2 30, PaO2 95, HCO3 18. **Nonexertional (classic) heat stroke; delirium superimposed on dementia again.** Temperature and mentation did not improve much after initial cooling and fluids in the ED, so he was admitted to the ICU (A3) |
| 2025-07-29 to 2025-08-05 | outside | ICU then medical floor (A3): continued active cooling and IV fluids; temperature normal by the next day; potassium replaced; sodium corrected slowly; amlodipine held while BP was low and resumed before discharge; mental status back to his baseline by discharge. Discharge labs Na 141, K 4.1, Cr 1.2. Discharged 2025-08-05 back to his nursing home, home drugs otherwise unchanged |
| 2025-09-16 | outside | the practice receives a short discharge summary after a records request (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The 2023 ED note gives no disposition or course for the delirium. The chart
  has him discharged home from the ED (A2), and the delirium clears to his baseline
  within about a month; the practice resolves it on 2023-12-14. It is diagnosed again
  by the hospital in July 2025, as part of the heat stroke.
- **R2.** The source's second encounter is dated October (practice date 2025-10-27)
  but says it happened "during a heatwave in July". It moves to 2025-07-29, a Tuesday,
  which falls after the last practice visit (2025-07-14). His age is 77 on that date,
  as the source says. "Two years after" the 2023 episode is about 20 months on the
  re-dated timeline; the hospital's note may say "about two years".
- **R3.** In 2023 his daughter brings him in and has watched him wander at night; in
  2025 he lives in a nursing home without air conditioning. The chart has him living
  with his daughter at joining and moving to a nursing home in March 2024, after night
  wandering at home continued. The practice remains his primary-care practice; the
  daughter brings him to office visits.
- **R4.** The 2023 ED note says only that his vital signs were stable: no numbers are
  carried for it. The source's "Complete blood count: within normal limits" and
  "Urinalysis: normal" are carried as normal panels.
- **R5.** The 2025 source plan stops after "do not improve significantly after initial
  cooling measures and fluid resuscitation". The chart carries the ICU course in A3 and
  never carries the unfinished sentence.
- **Left as the source has it:** no cause for the 2023 delirium is named, and no one
  changes his dementia or bladder drugs afterward. Nobody at the practice raises the
  facility's lack of air conditioning before the heat stroke.

## Additions (A): physician to confirm

- **A1.** 2023-08-22 new-patient visit (he arrives at the ED already carrying four
  chronic problems and six drugs; years with the practice is three).
- **A2.** Disposition of the 2023 ED visit: discharged home with his daughter the same
  night after the negative workup.
- **A3.** The heat-stroke admission's course after the source stops: ICU admission,
  continued cooling and fluids, potassium repletion for K 3.2, gradual sodium
  correction, amlodipine held while BP was low and resumed, mental status back to
  baseline, discharge 2025-08-05 to the nursing home with Na 141, K 4.1, Cr 1.2.
- **A4.** A post-discharge practice visit booked for 2025-08-12 is cancelled by the
  facility (transport); no practice visit follows (last seen stays 2025-07-14).
- **A5.** Housing status recorded as nursing home from March 2024 (R3).
- **A6.** Marital status widowed.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (colonoscopy before 75, shingles series, Tdap).
- Visits for blood pressure, dementia, knees and urinary symptoms about every 3 to 5
  months, with the daughter present; BP controlled on amlodipine 5 mg (state about
  134/80); weight steady around 74 kg, height 176 cm; basic metabolic panel, CBC,
  lipids yearly; TSH and vitamin B12 once at joining, normal. Creatinine around 1.0.
- A Medicare annual wellness visit in 2024 with a depression screen (low score) and
  advance care planning with the daughter as health care proxy.
- Influenza (high-dose) each fall at the practice in 2023 and 2024; updated COVID
  vaccine 2023 and 2024; PCV20 once; RSV once. After the move, the facility may give
  the fall 2025 vaccines; the practice hears of them only in passing.
- Refills, pharmacy traffic (a long-term care pharmacy after the move), facility forms,
  portal messages from the daughter, a missed visit, records requests.
- No visit booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Sparse. The 2023 ED visit reaches the practice as a short ED note faxed a week later,
with the BMP values, CT impression and diagnosis. The 2025 admission reaches it first as
the daughter's portal message, then as a short discharge summary received 2025-09-16
with the presenting vitals, key labs, diagnoses and discharge labs. After 2025-07-14
nothing is reconciled: the practice's problem and medication lists stay as they were at
that visit, and the hospital's diagnoses appear only in its records.

## Last seen

The practice last saw MRN00002311 on 2025-07-14. Practice contacts after that day do
not happen except messages, refills and records; outside care after it still does, and
its records are filed as received without changing the practice's lists. No visit is
booked after today.
