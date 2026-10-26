# Build requirements: unit-1858 (Alyssa Sears, MRN00001858)

Source: synthetic_hospital patient 1858, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who she is (S)

Alyssa Sears, born 1990-06-30, female, White, not Hispanic or Latino, English,
lives in Hatfield (01038), Blue Cross Blue Shield employer-sponsored PPO. Elementary
school teacher. Never smoker, no alcohol, no caffeine, no illicit drugs. No known drug
allergies. Surgical history: tonsillectomy at age 8, wisdom teeth extraction at age 24.
Family history: mother with autoimmune hypothyroidism (Hashimoto), maternal aunt with
Graves disease, father with hypertension.

Joined the practice on 2023-09-12 (A1) with no problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient preventive visit (A1): healthy, no complaints, no medications; height 165 cm, weight 62.4 kg; normal exam with a normal-size thyroid; screening labs normal including TSH (P) |
| 2024-10-08 | practice | annual physical (P): well; weight about 62 kg; normal exam |
| 2025-02-17 | practice | sick visit: 3 months of trouble sleeping, palpitations, feeling on edge, intermittent sweating; lost 5 kg (weight 57.4 kg) despite a bigger appetite. No caffeine, alcohol or drugs. HR 102, BP 130/85. Anxious-appearing; diffusely enlarged thyroid without nodules; tremor with arms extended; no exophthalmos; skin warm and moist. Same-day labs: TSH <0.01 mIU/L (0.4-4.0), free T4 3.0 ng/dL (0.9-1.7), total T3 250 ng/dL (80-180). **Graves disease diagnosed** that day on the labs, goiter and family history (R1); thyrotropin receptor antibody sent (A2); endocrinology referral placed |
| 2025-02-21 | practice | TRAb resulted: 6.8 IU/L (reference <1.75) (A2); result note to patient |
| 2025-04-28 | outside | endocrinology consult: 2 months of palpitations, heat intolerance, weight loss with increased appetite, tremor, frequent bowel movements. No medications. BP 150/80, HR 112; fine tremor of outstretched hands; warm moist skin; diffuse goiter. TSH <0.01 mIU/L, free T4 3.2 ng/dL (0.8-1.8), total T3 250 ng/dL (80-180). ECG: sinus tachycardia, otherwise normal. Treatment options reviewed; no drug started (R2). Letter reaches the practice late, 2025-06-16, after a records request (records gap) |
| 2025-08-25 | outside | endocrinology follow-up (she is 35, R3): 2 months of fatigue, palpitations, ongoing weight loss (56.0 kg), more sweating, heat intolerance, occasional tremor, irregular menstrual cycles, poor sleep; no recent illness, medication change or drug use. HR 118, BP 138/72; anxious, fine hand tremor; diffusely enlarged, non-tender thyroid. Labs: TSH suppressed and free T4 elevated (no numbers reach the practice). **Methimazole 10 mg daily started** (R4). Letter arrives 2025-10-20; the practice list lags until then (records gap) |
| 2026-03-06 | outside | endocrinology follow-up: months of worsening fatigue and muscle weakness, nearly 8 kg below her pre-illness weight despite eating more and snacking (weight 54.5 kg, BMI 20), feels hot even in air conditioning, sweating through the day, wired at night, more anxious and restless; no recent illness or travel. **Not taking any medications** (R5). Temperature normal, pulse 112 and regular, BP 128/76; warm moist skin, fine tremor both hands, brisk reflexes throughout; heart and lungs normal; no eye changes; no leg edema; diffusely enlarged, non-tender thyroid without nodules. TSH <0.01, free T4 3.8 ng/dL, total T3 240 ng/dL. **Methimazole restarted at 20 mg daily and propranolol 20 mg twice daily started** (A3); baseline CBC and liver panel normal (A3). Letter arrives 2026-03-11 |
| 2026-04-20 | outside | endocrinology labs (A4): TSH <0.01, free T4 2.2 ng/dL, total T3 192 ng/dL |
| 2026-05-11 | outside | endocrinology follow-up (A4): palpitations and sweating much better, taking methimazole daily; HR 82, BP 118/72, weight 56.6 kg; tremor gone. **Methimazole reduced to 10 mg daily; propranolol stopped.** Repeat labs in 8 weeks, which she may draw at the practice. Letter arrives 2026-05-18 |
| 2026-07-13 | practice | lab-only draw at endocrinology's request (A4): TSH 0.04 mIU/L, free T4 1.3 ng/dL, total T3 138 ng/dL, CBC normal; results forwarded to endocrinology |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps Graves disease as diagnosed on the first visit, with
  labs that show hyperthyroidism only. Resolution: the primary-care physician
  diagnoses Graves disease that day from the suppressed TSH, high free T4 and T3,
  the diffuse goiter and the family history, and sends an antibody (A2) to confirm.
- **R2.** Encounter 1 has no plan, and encounter 2 says she is only then starting
  treatment. Resolution: the April consult reviews antithyroid drug, radioactive
  iodine and surgery, and she leaves without a drug to think it over and return.
  No beta-blocker is started in February or April (see Left as the source has it).
- **R3.** Encounter 2 calls her 34; by the practice date 2025-08-25 she is 35. Its
  dates "January 15, 2020 and March 15, 2020" become February 2025 and April 2025.
- **R4.** Encounter 2 says "a medication" was started without naming it.
  Resolution: methimazole 10 mg daily, the standard first antithyroid drug.
- **R5.** Encounter 3 says she takes no medications, five months after one was
  started, and her levels are higher. Resolution: she was no longer taking the
  methimazole by March 2026. The chart does not explain why. The practice list
  carries methimazole 10 mg from the October 2025 letter until the March 2026
  letter, when it is stopped and restarted at the new dose.
- **R6.** Encounter 2's family history says no autoimmune disease in the family,
  against every other record. Not carried: the August letter's family history
  follows the profile.
- **R7.** Weights: 62.4 kg at joining (2023), about 62 kg in 2024, 57.4 kg in
  February 2025 (the 5 kg loss), 56.0 kg in August 2025, 54.5 kg in March 2026
  (BMI 20 at 165 cm; nearly 8 kg below her baseline), 56.6 kg in May 2026.
- **R8.** Free T4 reference ranges differ (0.9-1.7 at the practice's lab, 0.8-1.8 at
  endocrinology's). Both stay: different labs.

**Left as the source has it:** no beta-blocker for a heart rate of 102 to 118 from
February 2025 to March 2026; no treatment between the diagnosis and August 2025; the
elevated office blood pressures (130/85, 150/80) are not labeled hypertension and
nobody remarks on them; nobody addresses contraception or pregnancy planning before
methimazole. No note comments on these.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (the seed gives about 3 years with the
  practice; the 2025 sick visit is then an established-patient visit).
- **A2.** Thyrotropin receptor antibody 6.8 IU/L (reference <1.75), drawn
  2025-02-17, resulted 2025-02-21, to support the diagnosis the source asserts.
- **A3.** At the March 2026 visit, endocrinology restarts methimazole at 20 mg daily
  for free T4 3.8 and adds propranolol 20 mg twice daily for the pulse of 112, with
  baseline CBC and liver tests (normal), the standard next steps.
- **A4.** The usual monitoring after the restart: labs 2026-04-20 (TSH <0.01, free
  T4 2.2, total T3 192), visit 2026-05-11 (methimazole to 10 mg daily, propranolol
  stopped), labs 2026-07-13 at the practice (TSH 0.04, free T4 1.3, total T3 138,
  CBC normal). Next endocrinology visit is outside the chart.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results: a Pap with HPV cotest in 2021 normal, Tdap in 2019.
- Yearly physicals (2023 joining visit, 2024) with vitals, PHQ-2 or PHQ-9 (negative),
  routine labs (CBC, metabolic panel, lipids once, TSH in 2023). HIV and hepatitis C
  screening once (2023, nonreactive).
- Influenza vaccine each fall (2023, 2024, 2025), COVID vaccine in fall 2023 and 2024.
- Referral and records-request traffic for endocrinology (the April and August letters
  are slow to arrive), portal messages about results and scheduling, refill and
  pharmacy notes for methimazole and propranolol (prescribed by endocrinology), a
  telephone check-in after the March 2026 letter, a cancelled or rescheduled 2025
  physical.
- Booked visit after 2026-08-27: annual physical with a Pap/HPV cotest due.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
