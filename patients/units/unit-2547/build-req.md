# Build requirements: unit-2547 (Curtis Grant, MRN00002547)

Source: synthetic_hospital patient 2547, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its truncated sentences.

## Who he is (S)

Curtis Grant, born 2002-08-12, male, Black or African American, not Hispanic or
Latino, English-speaking, lives in South Hadley MA 01075, single. University student
health insurance plan. Undergraduate student and NCAA Division II cross-country
runner. Never smoker, no alcohol, denies drug use. NKDA. No surgical history.
Family history: father sickle cell trait; mother hypertension; paternal uncle type 2
diabetes. Sickle cell trait (HbAS), known since childhood.

Joined the practice 2022-08-23 (new patient visit, age 20). He brought sickle cell
trait and no medications. He takes no medications at any point in the chart.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-23 | practice | new patient / preventive visit (P): history as above, normal exam, pre-participation athletic form; sickle cell trait on the problem list (history) |
| 2024-09-07 (Sat) | outside, ED (R1) | **Exertional heat stroke** and **rhabdomyolysis**. Abrupt collapse in the final stretch of a cross-country race on a humid afternoon; beforehand severe dizziness, throbbing headache and vomiting, then loss of coordination. On arrival lethargic, intermittently agitated, unable to give his history. T 40.2 °C, HR 158, BP 96/58, RR 26. Skin hot, flushed and dry, not sweating; no focal neurologic deficit, no seizure activity. Mild transaminitis and elevated creatine kinase (no values given). Airway and circulation stabilized; rapid cooling and IV fluid resuscitation (A1). Observed in the ED overnight, discharged next day (A1) |
| 2024-09-09 | practice | patient portal message telling the practice (sparse record) |
| 2024-09-11 | practice | short ED note received (vitals and qualitative labs only, as above) |
| 2024-09-17 | practice | post-ED visit (P): feels well; normal exam; CMP and CK normal; heat stroke and rhabdomyolysis resolved; graded return to running with the team's athletic trainer, hydration and heat acclimatization counseling |
| 2025-06-07 (Sat) | outside, ED (R1) | **Exertional heat stroke** again, **rhabdomyolysis**, **acute kidney injury**. Friends bring him in after he collapsed at an outdoor summer music festival; about nine months after the first heat stroke; outside temperature 38 °C; he had danced for several hours without breaks and drunk little water. T 41 °C, HR 130, BP 90/60, RR 24. Disoriented and agitated; skin hot and dry. Na 145, K 4.2, creatinine 1.8, BUN 20, CK 10,000, PT 14 s (INR 1.2), PTT 35 s, lactate 5. Cooling and IV fluids (A1); observed overnight, discharged next day (A1) |
| 2025-06-09 | practice | patient phones the practice (sparse record) |
| 2025-06-16 | practice | short ED note received, with the labs above |
| 2025-06-17 | practice | post-ED visit (P): feels well, normal exam; BMP and CK normal; heat stroke, rhabdomyolysis and AKI resolved; heat-illness counseling repeated |
| 2026-04-14 (Tue) | outside, regional hospital admission (Internal Medicine) | **Seizure** (witnessed tonic-clonic at home, about 3 minutes, roommate's account, then a period of confusion), **rhabdomyolysis**, **acute kidney injury**. Age 23. Denies recent illness, drugs or alcohol; no medications. T 37.2 °C, BP 135/85, pulse 88, RR 16. Alert but fatigued; neurologic exam nonfocal. CK 20,000, creatinine 1.8, potassium 5.5. Urinalysis: specific gravity 1.025, pH 6.0, blood 3+, protein 1+, no RBCs on microscopy. CT head without contrast normal (A2). IV fluids; no antiseizure drug started (R3). Discharged 2026-04-16 with outpatient neurology follow-up advised |
| 2026-04-17 (Fri) | practice (last seen) | post-discharge visit (P): brings discharge papers; no further events, feels tired but well; normal exam, nonfocal neuro exam. **Neurology referral placed** (A3). Counseled no driving for six months after the seizure (Massachusetts rule), seizure safety, no training until labs normal. Repeat CK and BMP ordered for one week out |
| 2026-04-22 | practice | brief hospital discharge summary received, filed (lists not changed) |
| 2026-04-24 | practice | lab draw ordered at the last visit: CK and BMP normal (A4); PCP result note to patient |
| 2026-05 to 2026-08 | practice | portal messages only: neurology new-patient appointment is in September 2026; the neurology referral stays open today |

## Resolutions of source contradictions (R): physician to confirm

- **R1. Dates of the two heat strokes.** The source's practice dates put a
  cross-country race "on a humid afternoon" in late January and an "outdoor summer
  music festival" at 38 °C ambient in October, in New England. The second note says it
  came nine months after the first. Resolution: the race moves to 2024-09-07 (early
  cross-country season) and the festival to 2025-06-07, exactly nine months later. The
  admission stays 2026-04-14. The admission's "previously evaluated in 2020" becomes
  "in 2024 and 2025".
- **R2. Problem-list stamps.** The source's later notes carry heat stroke as an active
  problem. Resolution: each heat stroke is diagnosed at its ED visit and resolved at
  the post-ED practice visit; later notes mention it as history. Rhabdomyolysis is
  diagnosed at each of the three outside encounters (elevated CK at the first,
  CK 10,000 and 20,000 at the others) and AKI at the second and third (creatinine 1.8).
  The first two episodes resolve at the post-ED visits. The third episode's
  rhabdomyolysis, AKI and seizure stay on the practice list as entered at the
  admission, because the normal labs of 2026-04-24 come after the last visit.
- **R3.** The admission names no seizure drug and the source lists no medications.
  Resolution: no antiseizure drug is started; the discharge plan is outpatient
  neurology.
- **R4.** The source's truncated sentences (encounter 0 history and past history) are
  not carried. The source gives no CK or transaminase numbers at the first ED visit;
  the chart carries them only as "elevated" and "mild transaminitis".
- **Left as the source has it:** after two exertional heat strokes in an athlete with
  sickle cell trait, nobody orders heat-tolerance testing or a sports-medicine
  referral, and no note remarks on it. The seizure's cause is not settled in the
  chart.

## Additions (A): physician to confirm

- **A1.** Treatment and disposition at both ED visits: rapid cooling and IV fluid
  resuscitation (the source's "rhabdomyolysis requiring fluid resuscitation"),
  observation in the ED overnight and discharge the next day. No numbers beyond the
  source's.
- **A2.** CT head without contrast at the 2026 admission, no acute abnormality, the
  standard study after a first seizure.
- **A3.** Neurology referral at the 2026-04-17 visit (the discharge plan); the
  appointment is set for September 2026 and has not happened by today.
- **A4.** CK and BMP on 2026-04-24, within normal limits (creatinine normal).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from his prior pediatric practice (received 2022): immunizations completed in
  childhood and adolescence (Tdap 2014, HPV series, MenACWY with a booster, COVID
  primary series 2021), sickle cell trait from newborn screening.
- Yearly preventive visits each August (2023, 2024, 2025) with NCAA pre-participation
  forms that document sickle cell trait; vitals; PHQ-2 negative; lean runner (height
  178 cm, about 63 to 65 kg), low-normal resting pulse, normal blood pressure.
- Labs: CBC, CMP and lipid panel at joining, HIV and hepatitis C screening once
  (negative), CBC or CMP at later physicals, normal.
- Influenza vaccine each fall (nurse visits); COVID vaccine in 2022 and 2023, declined
  after; Tdap booster in 2024.
- Portal messages about forms, results and scheduling; a cancelled physical; athletic
  department paperwork.

No acute illnesses, injuries, new diagnoses, medications or life events beyond the
timeline.

## Last seen

The practice last saw MRN00002547 on 2026-04-17. Practice contacts after that day are
messages, the result of the draw ordered that day and records filed as received; no
practice visit, and the problem and medication lists stay as they were. Nothing is
booked.
