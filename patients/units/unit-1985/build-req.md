# Build requirements: unit-1985 (Derek Conti, MRN00001985)

Source: synthetic_hospital patient 1985, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the confidence-interval question,
the "you note that the planned sample size" stem).

## Who he is (S)

Derek Conti, born 1969-05-15, male, White, not Hispanic or Latino, English, lives in
South Hadley (01075), private PPO through his employer. High school history teacher.
Never smoker. Alcohol socially, 1 to 2 drinks on weekends. No known drug allergies.
No surgical history. Family history: father with type 2 diabetes diagnosed at 58;
mother with hypertension and hyperlipidemia; maternal grandfather with a myocardial
infarction at 65.

Joined the practice on 2019-09-10 (A1), moving from a prior primary-care practice.
He brought: essential hypertension and type 2 diabetes mellitus (both diagnosed at the
prior practice, years before; R1), on metformin 1000 mg twice daily and lisinopril
20 mg daily. Height 178 cm, weight about 95 kg (BMI about 30, steady throughout).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new-patient preventive visit (A1): history as above; BP near 132/84 on lisinopril; BMI 30; baseline labs (A1c near 6.5, creatinine 1.0, lipids near TC 210 / HDL 35 / LDL 145 / TG 150 with no lipid diagnosis or drug; R3); HCV and HIV screening; Tdap; colonoscopy referral (P) |
| 2021-12-30 | practice | annual physical "for ongoing diabetes management": feels well, no complaints; BP 132/85, HR 78, BMI 30; exam unremarkable; fasting glucose 126, A1c 6.5%, TC 210, HDL 35, LDL 145, TG 150. Metformin 1000 mg twice daily and lisinopril 20 mg daily continued unchanged (R1, R2) |
| 2022-07-01 | practice | follow-up for cholesterol and adherence: no new symptoms, adherent; BP 132/84; A1c 7.2%, fasting glucose 154, TC 210, LDL 140, HDL 40, TG 250. **Mixed hyperlipidemia diagnosed** (R3); diet and exercise counseling, recheck; no lipid drug (left as the source has it). Quiz dropped (R4) |
| 2022-12-19 | practice | fasting lab draw before the visit (R5): A1c 7.2%, fasting glucose 154, TC 210, LDL 130, HDL 40, TG 250 |
| 2022-12-26 | practice | follow-up: feels well, no new symptoms, adherent to metformin and lisinopril; BP 132/84; exam unremarkable; reviews the 12-19 labs; asks about a placebo-controlled trial of a new lipid-lowering drug he heard about; the physician discusses it, including that its planned enrollment is small (R4) |
| 2025-12-22 | practice | 2 weeks of fatigue, less urine and generalized swelling; urine turned dark brown. BP 160/100; 2+ pitting edema of both lower legs. Urinalysis: brown, red blood cell casts, protein 3+. Creatinine 2.8 (baseline 1.0), BUN 45, albumin 3.2, C3 low, C4 normal (R6). ASO titer sent (A2). **Acute poststreptococcal glomerulonephritis diagnosed** (R7); urgent nephrology referral (A3). Metformin and lisinopril unchanged |
| 2025-12-23 | practice | ASO titer resulted 640 IU/mL (reference under 200) (A2); physician calls him |
| 2026-01-07 | outside | nephrology consult (A3); no letter ever reaches the practice. He reports by portal on 2026-01-08: the nephrologist agreed it is post-strep, expects it to settle, no biopsy for now, repeat labs there |
| 2026-02-18 | outside | nephrology follow-up (A4): kidney function recovering, pressure still above goal, swelling better; **hydrochlorothiazide 25 mg daily started**. Visit summary reaches the practice late, 2026-04-06, after a records request |
| 2026-02-25 | outside | ED: weakness and palpitations for 2 hours; HCTZ started one week ago; no vomiting, diarrhea or laxatives; on metformin and lisinopril. BP 145/90, HR 102; mild tremor. K 2.9, Na 140, Cl 100, bicarbonate 28, glucose 180, creatinine 1.0. ECG: U waves and flattened T waves. **Hypokalemia diagnosed**; oral replacement, tolerating by mouth (A5); **HCTZ stopped** (A5); **potassium chloride started** (A5). ED record reaches the practice 2026-03-04 |
| 2026-02-26 | practice | he calls about the ED visit; post-ED visit booked (P) |
| 2026-03-02 | practice | post-ED visit (A6): palpitations and weakness gone; finished potassium; BP 142/88; K 3.9. **Hypokalemia resolved**. **Amlodipine 5 mg daily started** (A6). Records requested from nephrology |
| 2026-05-11 | practice | fasting draw before the physical (A7, P): urinalysis negative for blood and protein; C3 112 mg/dL; A1c, BMP, lipids, urine albumin/creatinine; all resulted 2026-05-12 |
| 2026-05-14 | practice | annual physical, last seen (A7): feels well, no swelling, urine clear; BP 128/80; reviews the 05-11 results. **Acute poststreptococcal glomerulonephritis resolved** (A7) |
| 2026-09-17 | practice | booked routine diabetes follow-up (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls encounter 0 "ongoing diabetes management" and lists
  metformin as a home medication, yet stamps type 2 diabetes "diagnosed" that day.
  Resolution: type 2 diabetes was diagnosed at his prior practice before he joined
  and is on his list from 2019-09-10 (onset left vague, a few years before). The A1c
  6.5% and fasting glucose 126 in December 2021 are on-treatment values. The stamp is
  dropped.
- **R2.** Encounter 0 has a garbled line that he "takes hydrochlorothiazide", while
  the ED note says a thiazide was "newly prescribed" one week before 2026-02-25.
  Resolution: he takes no thiazide until 2026-02-18; the encounter-0 mention is
  dropped. Hypertension is treated with lisinopril 20 mg alone until then.
- **R3.** Lipids were already abnormal in December 2021 (LDL 145) and in his 2019
  baseline, but the source stamps mixed hyperlipidemia on encounter 1. Resolution:
  the diagnosis is recorded on 2022-07-01, when triglycerides first run 250. Before
  that, lipid results are filed without a diagnosis or comment. The lab reports LDL
  by direct measurement (the calculated value from TC, HDL and TG would be lower);
  no one remarks on it.
- **R4.** Encounter 1's plan is a confidence-interval quiz and encounter 2's plan
  stops mid-sentence. Resolution: the July 2022 plan is diet, exercise and a recheck.
  In December 2022 the physician gives a balanced view of the trial (placebo-
  controlled, small planned enrollment, worth reading the consent and asking the
  study team questions); he takes the information home. No enrollment is ever
  recorded and nothing further is said of it. No study figures are carried.
- **R5.** Encounter 2 reviews "recent" labs that repeat encounter 1's values except
  LDL. Resolution: a separate fasting draw on 2022-12-19 with the values listed. The
  repeat values are carried as they are.
- **R6.** The source gives C3 as "low" and C4 as "normal" with no numbers.
  Resolution: the lab reports C3 62 mg/dL (reference 90 to 180) and C4 24 mg/dL
  (reference 10 to 40). The urinalysis that shows red cell casts also reports
  dipstick blood large and red cells too numerous to count, which the casts and
  brown color imply.
- **R7.** The source stamps APSGN on encounter 3 with no evidence of strep.
  Resolution: the diagnosis is recorded on 2025-12-22 from the nephritic picture and
  low C3, supported the next day by the ASO titer (A2). No preceding sore throat or
  skin infection is recorded; nobody asks after one in the chart.
- **R8.** A creatinine of 2.8 is managed as an outpatient. Resolution: kept as the
  source has it, with the urgent nephrology referral (A3); no admission.
- **Left as the source has it:** no statin or other lipid-lowering drug is ever
  started, though he is diabetic with LDL 130 to 145; no note comments on it. A1c
  6.5% then 7.2% brings no diabetes drug change. At the 2025-12-22 visit metformin
  and lisinopril continue unchanged with a creatinine of 2.8 and BP 160/100. No
  primary-care visit after the source revisits the statin gap.

## Additions (A): physician to confirm

- **A1.** The 2019-09-10 new-patient visit (the first source encounter is an
  established-patient physical; seed: 7 years with the practice).
- **A2.** ASO titer 640 IU/mL (reference under 200), drawn 2025-12-22, resulted
  2025-12-23: the evidence for the source's poststreptococcal diagnosis.
- **A3.** Urgent nephrology referral on 2025-12-22 and the consult on 2026-01-07,
  known only from his portal message (supportive care, no biopsy, expected to
  settle, repeat labs at nephrology). No numbers from nephrology reach the chart.
- **A4.** Nephrology follow-up on 2026-02-18 that starts hydrochlorothiazide 25 mg
  daily for persistent high pressure and edema (the source's "newly prescribed
  thiazide one week ago"); its short summary (kidney function recovering, pressure
  above goal, swelling better, HCTZ started, lisinopril continued) arrives late.
- **A5.** ED treatment of the potassium of 2.9: potassium chloride 40 mEq by mouth in
  the ED, tolerated; hydrochlorothiazide stopped; discharged on potassium chloride
  20 mEq extended-release twice daily for 5 days; PCP follow-up in a few days.
- **A6.** The 2026-03-02 post-ED visit: symptoms gone, potassium 3.9, BP 142/88
  off the thiazide; amlodipine 5 mg daily started; hypokalemia resolved.
- **A7.** The 2026-05-14 physical: BP 128/80 on lisinopril and amlodipine;
  the 2026-05-11 draw shows urinalysis negative for blood and protein and C3 112 mg/dL; glomerulonephritis
  recorded resolved.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries (an ECG's rate and intervals), but
  nothing that adds a territory, a severity grade or a symptom, or changes the diagnosis
  or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results.
- Diabetes and blood pressure visits about every 6 months (with gaps, a cancelled visit
  in spring 2020 and a missed one in late 2023), yearly physicals, A1c, metabolic panel,
  lipids, urine albumin/creatinine (normal, about 10 to 15 mg/g), foot exams with normal sensation. Levels
  between the source's draws stay near the nearest source values: A1c near 6.5% before
  2022 and near 7.2% after; LDL near 130 to 145; BP near 132/84 on lisinopril until
  late 2025.
- Diabetic eye exam letters from an optometrist with no retinopathy, in some years
  only (records gaps: some years he says he went and no letter comes).
- Prevention: screening colonoscopy in November 2019 at an outside GI group, normal,
  repeat in 10 years; influenza vaccine each fall; COVID vaccines (primary series in
  spring 2021 reported by him, later doses at the practice); Shingrix series
  2020 to 2021; PCV20 in 2023 for diabetes; Tdap at joining. Hepatitis C and HIV
  screening once, negative. PHQ-2 at physicals, negative.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling,
  records requests, one cancelled and one missed visit.
- After 2026-05-14: result message, refills, scheduling. Booked visit 2026-09-17: a
  routine diabetes follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001985 on 2026-05-14. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
