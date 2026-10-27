# Build requirements: unit-2001 (Kevin Allard, MRN00002001)

Source: synthetic_hospital patient 2001, in `source.md` beside this file. This file and
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

## Who he is (S)

Born 1969-11-15, male, White, not Hispanic or Latino, English, commercial insurance,
lives in Easthampton. Landscaping supervisor. Current smoker, 1 pack per day, about
30 pack-years by 2024. Alcohol: occasional social use, 1 to 2 drinks a month. No known
drug allergies until the May 2024 admission (R2). Family history: father MI at 62,
mother hypertension, brother hyperlipidemia.

Joined the practice on 2022-09-13 (A1), bringing essential hypertension on
**lisinopril 20 mg daily**, hyperlipidemia on **atorvastatin 40 mg daily**, coronary
artery disease (A1) and tobacco use disorder (nicotine dependence, cigarettes). No
surgical history at joining; no peptic ulcer history at joining (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit (A1): history as above; NKDA; smoking counseling, not ready to quit; screening labs; lung cancer screening ordered (A7); prior-practice records requested |
| 2022-09 to 2022-10 | outside | prior-practice records received (A1): 2021 CT coronary angiography, colonoscopy 2019 normal, Tdap 2018 |
| 2022-10 | outside | low-dose CT chest, Lung-RADS 1 (A7) |
| 2024-05-07 to 2024-05-13 | outside | regional hospital admission under general surgery. Arrives after hematemesis, pale, diaphoretic, hypotensive; IV fluid resuscitation. Endoscopy shows a **bleeding gastric ulcer** and he goes to **operative repair of the bleeding gastric ulcer**. During surgery he receives 2 units of packed red cells; minutes into the second unit: chills, shortness of breath, diffuse flank pain, a further fall in blood pressure, restlessness and tachycardia. Dark red urine in the catheter; scleral icterus over the next hours. Labs: rising creatinine, raised plasma free hemoglobin, raised indirect bilirubin, positive direct antiglobulin test, fragmented red cells on the smear (values in R4). **Acute hemolytic transfusion reaction**; ICU monitoring with supportive care; recovers (R9). **Peptic ulcer disease diagnosed** (R1). **Blood products recorded as an allergy** (acute hemolytic transfusion reaction) (R2). Ulcer biopsies benign, H. pylori negative (A2). **Pantoprazole 40 mg twice daily started** at discharge (A2). Discharge summary received by the practice |
| 2024-05-21 | practice | post-discharge visit (P): values in R4; transfusion reaction resolved (R3) |
| 2024-07-16 | outside | GI repeat upper endoscopy (A2): healed ulcer scar, biopsies benign; **pantoprazole stepped down to 40 mg daily**, continued long term |
| 2025-04-22 | outside | regional hospital ED: 2 hours of severe crushing chest pain to the left arm with shortness of breath, nausea and sweating; known CAD; admitted with hematemesis the year before (R5); smokes a pack a day. BP 150/90, HR 98, RR 22, SpO2 94% on room air. Diaphoretic; regular rhythm, no murmur, rub or gallop. Troponin I 0.5 ng/mL (normal <0.04), CK-MB 25 U/L (normal 0-24), CBC and basic metabolic panel normal. Chest x-ray unremarkable. ECG: ST elevation in II, III and aVF. **ST-segment elevation myocardial infarction, inferior wall** |
| 2025-04-22 to 2025-04-25 | outside | cardiology admission (A3): emergent catheterization, culprit occlusion of the right coronary artery, one drug-eluting stent. Echo LVEF 55% with inferior wall hypokinesis. **Aspirin 81 mg daily, clopidogrel 75 mg daily, metoprolol succinate 25 mg daily, sublingual nitroglycerin 0.4 mg as needed started; atorvastatin 40 mg stopped and atorvastatin 80 mg daily started**; lisinopril 20 mg and pantoprazole 40 mg continue. Cardiac rehab referral. Discharge summary received |
| 2025-04-29 | practice | post-discharge visit (P) |
| 2025-05 to 2025-08 | outside | cardiology follow-up letter (A6); cardiac rehabilitation completed (A4) |
| 2025-10-23 | outside | regional hospital ED: 45 minutes of heavy central chest pressure that began while mowing a lawn, to the left arm and jaw; known CAD and the April inferior MI (R5); diaphoretic and anxious; 30 pack-year history. BP 150/90, HR 98, RR 22, SpO2 94% on room air (R7). Regular rhythm, no murmur, rub or gallop; lungs clear. Troponin I 0.5 ng/mL, CK-MB 25 U/L, CBC and electrolytes normal (R7). Chest x-ray unremarkable. ECG: ST elevation in II, III and aVF. **Recurrent ST-segment elevation myocardial infarction, inferior wall** |
| 2025-10-23 to 2025-10-25 | outside | cardiology admission (A5): catheterization, new severe stenosis in the proximal right coronary artery with the April stent patent, one drug-eluting stent; dual antiplatelet therapy to continue 12 months from this event; other medicines unchanged. Discharge summary received |
| 2025-10-30 | practice | post-discharge visit (P) |
| 2026-07-22 | practice | last visit (seed `last_seen`): routine follow-up, labs drawn and resulted |
| after 2026-08-27 | practice | booked routine follow-up (seed `booked`) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists peptic ulcer disease and the ulcer repair from the start, but
  the first encounter is the bleed that finds the ulcer and the surgery that repairs it.
  Resolution: peptic ulcer disease is diagnosed and the repair done during the May 2024
  admission. Practice notes before May 2024 carry no ulcer history and no surgical history.
- **R2.** The blood-products allergy appears on the first encounter's allergy list, before
  the reaction that caused it. Resolution: it is recorded at the May 2024 admission
  (substance: blood products; reaction: acute hemolytic transfusion reaction; severe,
  high criticality). Before that he has no known drug allergies.
- **R3.** Later source lists carry the transfusion reaction as an active problem with a
  diagnosis stamp. Resolution: diagnosed 2024-05-07, resolved at the 2024-05-21 practice
  visit once kidney function and bilirubin are back to normal; it remains in the history
  and on the allergy list. Outside ED records in 2025 may still copy it into their lists.
- **R4.** The source gives the admission labs only as directions. The discharge summary
  carries: hemoglobin 7.4 g/dL on arrival and 9.1 at discharge; creatinine 1.08 mg/dL on
  arrival, 2.36 at peak on 2024-05-08, 1.42 at discharge; indirect bilirubin 3.4 mg/dL;
  plasma free hemoglobin 85 mg/dL; direct antiglobulin test positive. At the 2024-05-21
  practice visit: hemoglobin 10.6, creatinine 1.02, total bilirubin 0.9. Hemoglobin back
  in the normal range by the September 2024 physical. No other admission numbers.
- **R5.** Years stated in source text follow the re-dated timeline: the "prior 2020
  admission for hematemesis" is the May 2024 admission; the "inferior wall MI in January
  2021" is the April 2025 MI.
- **R6.** Both ED notes list only lisinopril and atorvastatin. Resolution: the ED
  records keep their short copied lists (pantoprazole missing in April 2025; aspirin,
  clopidogrel, metoprolol, nitroglycerin, pantoprazole and the 80 mg dose missing in
  October 2025), as copied outside lists do. The practice's lists follow the course above.
- **R7.** The two ED visits give identical vital signs and identical troponin and CK-MB
  values. Each ED record carries them as the source gives them.
- **R8.** The MI problem: each STEMI is diagnosed at its ED visit. After the acute phase
  the practice list carries it as old myocardial infarction (resolved the acute entry at
  the 2025-07 visit, and again at the 2026-01 visit after the second event), and presence
  of a coronary artery stent is recorded from April 2025.
- **R9.** The admission plan stops mid-sentence at ICU monitoring. Resolution: ICU
  supportive care, recovery, discharge home on hospital day 7 (2024-05-13). Lisinopril
  was held in hospital and resumed before discharge.
- **R10.** Coronary artery disease is listed from the start and called "known" at the
  first ED visit, with no evidence in the source. Resolution: A1's 2021 CT coronary
  angiography.
- **Left as the source has it:** no antiplatelet drug before the April 2025 MI despite
  known coronary disease; nobody raises one and no note remarks on it. He keeps smoking a
  pack a day through today: counseling is offered at visits, he declines medication, no
  smoking-cessation drug is ever started.

## Additions (A): physician to confirm

- **A1.** The 2022-09-13 new-patient visit (the source's first encounter treats him as an
  established patient with known disease). Prior-practice records: CT coronary
  angiography in 2021 showing nonobstructive plaque in the right coronary artery (about
  40%), medical therapy; screening colonoscopy 2019 without polyps, repeat in 10 years;
  Tdap 2018.
- **A2.** The ulcer's usual course: surgical pathology from the repair shows a benign
  gastric ulcer, no malignancy, no H. pylori; pantoprazole 40 mg twice daily from
  discharge; general surgery clinic check 2024-05-28, incision healing, no restrictions
  after six weeks; GI repeat endoscopy 2024-07-16 shows a healed ulcer scar with benign
  biopsies, pantoprazole to 40 mg daily long term.
- **A3.** The first STEMI's usual course: emergent catheterization with one
  drug-eluting stent to an occluded right coronary artery; admission to 2025-04-25; echo
  LVEF 55% with inferior wall hypokinesis, no significant valve disease; discharge on
  aspirin 81 mg, clopidogrel 75 mg, metoprolol succinate 25 mg, atorvastatin 80 mg and
  nitroglycerin 0.4 mg sublingual as needed, with lisinopril 20 mg and pantoprazole 40 mg
  continued; cardiac rehab referral.
- **A4.** Cardiac rehabilitation, 12 weeks, May to August 2025, completed; completion
  letter received.
- **A5.** The second STEMI's course: catheterization finds a new severe proximal right
  coronary stenosis with the April stent patent; one drug-eluting stent; discharge
  2025-10-25 with medicines unchanged and dual antiplatelet therapy planned through
  October 2026.
- **A6.** Outpatient cardiology follow-up letters (2025-05, 2025-12, 2026-05): no angina,
  medicines unchanged, smoking cessation urged.
- **A7.** Yearly low-dose CT lung cancer screening (current smoker, 30 pack-years, age
  52 to 56): 2022, 2023, 2024 and 2025, each Lung-RADS 1.
- **A8.** On atorvastatin 80 mg the LDL falls to the 60s (from about 100 on 40 mg).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Blood pressure, lipid and coronary follow-up visits every 3 to 6 months; physicals each
  September; BMP or CMP, CBC, lipids; A1c screening; hepatitis C and HIV screening once
  (negative). Blood pressure runs mildly above goal on lisinopril alone (high 130s/80s)
  and near goal after metoprolol is added. Weight steady around 94 kg, height 180 cm.
- Influenza vaccine each fall, updated COVID vaccines, Shingrix series (2023), PCV20
  (smoker). Depression and alcohol screens normal. Smoking counseling at visits.
- Post-discharge visits after each admission; refills, pharmacy traffic, portal messages
  about results, refills and scheduling, a missed visit.
- Booked visit after 2026-08-27: routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002001 on 2026-07-22. Practice contacts after that day are
only messages, refills and results; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after today.
