# Build requirements: unit-1913 (Abigail O'Leary, MRN00001913)

Source: synthetic_hospital patient 1913, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-like phrasing.

## Who she is (S)

Born 1996-11-04, female, White, not Hispanic or Latino, English-speaking, private PPO
through a parent's employer plan. College student when she joins, then a graduate
student in library sciences. Never smoker, no alcohol, no recreational drugs. No known
drug allergies. No surgical history. No medications at any point. Family history:
mother with mitral valve prolapse and mild mitral regurgitation; maternal grandmother
with a "heart murmur" never worked up; father with hypertension; no family history of
sudden cardiac death. Thin build: height 165 cm, weight about 52 kg (BMI about 19),
steady throughout.

Joined the practice in 2017 (A1: new-patient visit 2017-08-29, age 20), healthy, no
problems and no drugs on joining. Single.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2017-08-29 | practice | new-patient visit and physical (A1); history as above; normal exam; CBC and BMP normal; flu vaccine |
| 2018-09-11 | practice | annual physical (P); first cervical cytology at 21, negative (NILM); Tdap (10 years after the adolescent dose in the pediatric record) |
| 2019-09-10 | practice | annual physical (P); HIV and hepatitis C screening negative; lipid panel normal |
| 2020-06-22 | practice | office visit: about 6 months of intermittent palpitations, a sense of the heart skipping beats; no chest pain, dyspnea or syncope; no medications; BP 120/75, HR 78, RR 16; high-pitched click then a mid-to-late systolic murmur at the apex radiating to the axilla, softer on squatting from standing; office ECG normal sinus rhythm without significant ST-T changes. CBC, BMP and TSH normal (P). **Mitral valve prolapse diagnosed clinically** (R1); referred to cardiology for evaluation and echocardiogram |
| 2021-01-04 | outside | cardiology consult (outpatient): palpitations, usually while sitting quietly and better with activity, plus occasional chest discomfort; vitals normal; 2/6 high-pitched mid-systolic click and late-systolic murmur at the apex, no radiation; exam otherwise unremarkable; echocardiogram: mitral valve billows into the left atrium in systole. MVP confirmed. Letter received late, 2021-03-09, after a records request (R4) |
| 2022-06-28 | outside | cardiology telehealth follow-up: episodes at rest, not with exertion, resolve on their own; no syncope, fever, recent illness or weight loss; no family history of sudden cardiac death; vitals reported normal; 2/6 high-pitched mid-systolic click and late-systolic murmur at the apex, louder with Valsalva and standing; no edema or JVD. Repeat echocardiogram to be scheduled. **No letter ever reaches the practice**; the practice knows of the visit only from the patient (R4) |
| 2023-12-22 | outside | cardiology follow-up (outpatient), age 27 (R2): intermittent palpitations and brief sharp chest discomfort, at rest, not with exertion; no syncope, dizziness or recent illness; no medications; vitals normal; 2/6 high-pitched mid-systolic click and late-systolic murmur at the apex; no JVD, edema or cyanosis. Letter received 2024-01-16 |
| 2025-09-16 | practice | annual physical (P), **last seen**: palpitations come and go as before; known MVP; click heard; CBC, CMP and lipid panel normal; flu vaccine |
| 2025-12-21 | outside | ED visit (a Sunday): intermittent palpitations and spells of lightheadedness at rest, sometimes with a vague chest discomfort; vitals normal; thin build and mild scoliosis on exam; no murmur heard supine; ED ECG sinus rhythm and basic labs normal (P); the ED physician's impression is mitral valve prolapse (R3); an outpatient echocardiogram is to be scheduled; discharged home. The patient tells the practice by portal message on 2025-12-23; the ED record is received 2026-02-10 after a request and filed without changing the practice's lists |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps MVP as diagnosed at the Internal Medicine visit but confirms
  it by echocardiogram only at cardiology. Resolution: the practice records mitral
  valve prolapse on 2020-06-22 as a clinical diagnosis from the click-murmur exam and
  the family history, and refers for echocardiogram; the cardiology echo on 2021-01-04
  confirms it. No mitral regurgitation grade is ever given for the patient: the
  practice receives only the cardiology letter's description of the echo, not a
  separate echo report.
- **R2.** The source calls her 26 at the third cardiology visit; by the seed's birth date
  she is 27 on 2023-12-22. Years in source text follow the re-dated timeline:
  cardiology's "evaluated in 2020" (2022 visit) reads as first seen by primary care in
  2020 and cardiology in early 2021; "2020 and 2022" (2023 visit) reads as 2021 and 2022
  at cardiology.
- **R3.** The ED "suspects" MVP and schedules an echo to confirm it, although MVP has been
  known for five years. Resolution: the ED physician works without her outside records;
  the ED note takes her report of known MVP, calls the presentation consistent with it,
  and advises an outpatient echocardiogram with her cardiologist. No murmur supine in
  the ED and the axillary radiation heard in 2020 but not by cardiology are
  interobserver and positional differences, kept as written.
- **R4.** Records are gappy (seed texture): the January 2021 cardiology letter arrives two
  months late after the patient mentions the visit; the June 2022 telehealth note never
  arrives (the 2022 physical records the patient's account); the December 2023 letter
  arrives in three weeks; the ED record arrives seven weeks late after a request.
- Practice exams before June 2020 record a regular rhythm and do not comment on a murmur
  or click; no practice exam comments on the spine.
- **Left as the source has it:** no medication is ever started for the palpitations; no
  ambulatory rhythm monitor is ever ordered; the repeat echocardiogram planned in June
  2022 has no result anywhere in the chart and the December 2023 letter does not
  mention it; the echo the ED advised has no result by today. Nobody comments on any of
  these.

## Additions (A): physician to confirm

- **A1.** The 2017 new-patient visit (nine years with the practice by the seed; the 2020
  visit is a problem visit for an established patient). Pediatric records received
  2017-10 list childhood vaccines complete, including the HPV series and adolescent
  Tdap, with nothing else of note.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Yearly physicals in September (2017 to 2025; the fall 2020 physical cancelled by the
  patient, a 2024 physical missed and rebooked), PHQ-2 screens 0 to 1, BMI about 19.
- Cervical cytology at 21, 24 and 27 (2018, 2021, 2024), all negative.
- Influenza vaccine each fall (2020 at a nurse visit); COVID mRNA vaccines from spring
  2021 (primary series and later doses at pharmacies, entered from the registry or the
  patient's card); Tdap 2018. HIV and hepatitis C screening once (2019).
- Labs at the physicals: CBC and BMP 2017, lipid panel 2019, CBC, CMP and lipid panel
  2025; all normal.
- Portal messages about results, scheduling, forms for school, records requests, the
  cardiology referral scheduling, a cancelled and a missed visit.
- After the last visit: the portal message about the ED, the ED record filed, and
  scheduling messages only.
- Booked visit after 2026-08-27: annual physical on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001913 on 2025-09-16. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
