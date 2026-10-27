# Build requirements: unit-1867 (Andrea Brooks, MRN00001867)

Source: synthetic_hospital patient 1867, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the laptop offered "as a gift" is
carried only as the clinical observation in the timeline).

## Who she is (S)

Andrea Brooks, born 1989-07-23, female, White, not Hispanic or Latino, English, Medicaid,
lives in Granby. Retail sales associate. Never smoker, no alcohol, no substance use.
Allergies: no known drug allergies. Surgical history: wisdom teeth extraction at age 18
(about 2007). Family history: mother with bipolar I disorder; maternal grandmother with
schizophrenia. A sister is involved in her care (she brings her to the ED and gives
history at the admission).

Joined the practice in 2022 (A1: new-patient visit 2022-06-16, age 32). She brought
bipolar I disorder, followed by an outside psychiatrist who prescribes her drugs:
lithium carbonate 600 mg by mouth twice daily and quetiapine 300 mg by mouth at bedtime
(both recorded as historical, prescribed by psychiatry). The source gives no year of
diagnosis; the chart gives none.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-06-16 | practice | new-patient visit (A1): history as above; stable on lithium and quetiapine through outside psychiatry; baseline labs (P) |
| 2022-06-16 to 2025-02-25 | practice | primary-care layer (P): lithium monitoring, physicals, vaccines; all results normal, lithium levels in range |
| 2025-05-08 | outside | outpatient psychiatry follow-up: cheerful, elevated mood, "on top of the world"; sleeping 3 to 4 hours a night for a week without feeling tired; recent large purchases she cannot afford; rapid speech, easily distracted; offered the psychiatrist an expensive laptop as a gift, which was declined. Assessment: bipolar disorder, current episode manic. Lithium 600 mg twice daily and quetiapine 300 mg at bedtime unchanged (R2). The practice receives this note late, on **2025-08-20**, after a records request (R5) |
| 2025-06-06 | outside | ED, brought by her sister: 2 weeks of erratic behavior, grandiose plans to start a business, online spending she cannot afford; feels "on top of the world"; sleeping 3 hours a night; more talkative per sister; about one month after the psychiatry visit. Vitals normal. Well-groomed, rapid speech, distractible by things in the room, thought process goal-directed, denies suicidal ideation, understands the need for treatment and wants to avoid hospitalization; physical exam otherwise unremarkable. CBC, basic metabolic panel and thyroid tests normal. Diagnosis: **bipolar I disorder, current episode manic, moderate**. Discharged home with her sister to outpatient psychiatry (R3). ED record received 2025-06-10; the ED diagnosis goes on the practice problem list from that record |
| 2025-07-01 | practice | **last seen**: post-ED follow-up (A2). Reviews ED record; patient reports the May psychiatry visit; still followed by psychiatry, same doses. Lithium level, BMP, TSH, CBC (P, normal; lithium in range). Records request to psychiatry |
| 2025-11-19 to 2025-12-04 | outside | inpatient psychiatry admission (end date R4): a month of auditory hallucinations and delusional belief that she is a prophet with a special mission to save the world, with significant social impairment; intermittent stretches of several days of elevated mood, increased energy and decreased need for sleep; her sister reports weeks of relatively good function between mood episodes while the delusions and hallucinations persist. No substance use. Physical exam unremarkable. CBC, comprehensive metabolic panel and thyroid tests normal. Discharge diagnosis **schizoaffective disorder, bipolar type** (R1). Lithium 600 mg twice daily and quetiapine 300 mg at bedtime continued (R2); outpatient psychiatry follow-up. Age 36 (R6) |
| 2025-12-10 | practice (message) | patient tells the practice by portal she was hospitalized and is home; PCP requests the discharge summary (texture) |
| 2026-01-22 | practice (records) | discharge summary received and filed. The practice's problem and medication lists are not updated (after last_seen) |
| 2026-09-10 | practice | booked visit after today: follow-up and physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists "Bipolar I Disorder", "bipolar disorder, current episode manic"
  and "Bipolar I Disorder, Current Episode Manic, Moderate" with diagnosed stamps, and
  "Schizoaffective disorder" as a primary diagnosis that appears on no problem list.
  Resolution: bipolar I disorder is known from joining; the psychiatry note of 2025-05-08
  assesses a current manic episode; the ED of 2025-06-06 names it bipolar I disorder,
  current episode manic, moderate, which the practice records from the ED record and reviews on 2025-07-01. Schizoaffective
  disorder, bipolar type, is the inpatient discharge diagnosis (psychotic symptoms
  persisting between mood episodes). It appears only in the discharge summary; the
  practice list keeps bipolar I because nothing is reconciled after last_seen.
- **R2.** The source keeps lithium 600 mg twice daily and quetiapine 300 mg at bedtime on
  every list through two manic presentations and a psychotic admission, and states no
  plan. Resolution: the doses are unchanged at every encounter, including discharge.
- **R3.** The ED note gives no disposition; she wants to avoid hospitalization, denies
  suicidal ideation and understands the need for treatment. Resolution: discharged home
  with her sister to outpatient psychiatry.
- **R4.** The source gives no length of stay. Resolution: discharged 2025-12-04.
- **R5.** The source's outpatient psychiatry visit precedes the practice's knowledge of
  it. Resolution (records with gaps): the practice learns of it from the ED record and the
  patient on 2025-07-01, and the psychiatry note itself arrives 2025-08-20. Earlier
  psychiatry visits send no letters.
- **R6.** The source calls her 35 at the admission; by the seed's birth date she is 36 on
  2025-11-19. Its "January and February 2020" become May and June 2025.
- The source's laptop gift is carried as a clinical observation (she offered an expensive
  gift; it was declined), not as a question.
- **Left as the source has it:** no drug or dose change for two manic episodes and a
  psychotic admission; no lithium level at the ED or in the admission's labs. No note
  comments on either.

## Additions (A): physician to confirm

- **A1.** 2022-06-16 new-patient visit (the source's first encounter is a psychiatry
  follow-up of an established illness, so she had a primary-care home before it).
- **A2.** 2025-07-01 post-ED practice visit (the seed's last_seen).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results (a normal Pap in 2020).
- Lithium monitoring about every 6 months: lithium level in the usual range (0.6 to 1.0
  mmol/L), BMP with normal creatinine and calcium, TSH normal. Yearly CBC, lipids and A1c
  for quetiapine metabolic monitoring, normal. Weight stable, BMI about 27 (height 165 cm).
  Psychiatry prescribes both drugs; the practice records them as historical and shares
  results with psychiatry.
- Yearly physicals (2023, 2024, 2025), with one missed and rescheduled; a cervical
  cytology with HPV co-test in 2023, normal; PHQ-9 screens low; HIV and hepatitis C
  screening once, negative; influenza and COVID vaccines each fall 2022 to 2024; Tdap
  once (2023).
- Refills and pharmacy traffic, requests from the psychiatry office for lab copies, portal
  messages about results and scheduling, a records request.
- Booked visit after 2026-08-27: follow-up and physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001867 on 2025-07-01. Practice contacts after that day are
only messages, refills, results of that day's draw and outside records filed as received;
the practice's lists stay as they were. A visit is booked after today.
