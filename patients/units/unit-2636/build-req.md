# Build requirements: unit-2636 (Nathan Webster, MRN00002636)

Source: synthetic_hospital patient 2636, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("Gastric biopsy is shown below").

## Who he is (S)

Born 1988-01-31, male, White, not Hispanic or Latino, English-speaking, commercial PPO,
lives in Amherst. Construction laborer. Current every-day cigarette smoker since about
age 19, 1 to 1.5 packs per day, 22.5 pack-years as of the 2024 vascular visit (R3).
No alcohol, no illicit drugs. No known drug allergies. Surgical history: tonsillectomy
at age 8. Family history: father with autosomal dominant polycystic kidney disease and
end-stage kidney disease; mother with emphysema/COPD, a former smoker; one older brother
(37 in 2023), diagnosed with alpha-1 antitrypsin deficiency in spring 2023 (R2).

Joined the practice in September 2019 (A1), age 31. Brought: tobacco use disorder
(nicotine dependence, cigarettes); ibuprofen over the counter 400 to 600 mg every 6 to
8 hours as needed for work-related muscle aches (historical). No prescriptions. No
kidney, lung or vascular diagnosis is known at joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-24 | practice | new-patient visit (A1): history as above; height 178 cm, weight 82 kg; BP normal; exam normal; smoking cessation advised, not ready (P) |
| 2022-05-09 | practice | visit (A2): headaches and intermittent palpitations for about 6 months; BP 155/95; exam: bilateral flank masses, soft bruit over the left upper quadrant; ECG normal sinus rhythm (A2). Also mild breathlessness when exerting at work: **albuterol 90 mcg MDI 2 puffs every 4 to 6 hours as needed started** (R4). Ordered: BMP, CBC, TSH, aldosterone and plasma renin activity, renal ultrasound |
| 2022-05-10 | practice lab | creatinine 1.2 mg/dL; aldosterone 21 ng/dL, mildly elevated (R5); plasma renin activity 3.4 ng/mL/h (A2); TSH, CBC and the rest of the BMP normal |
| 2022-05-17 | outside imaging | renal ultrasound: multiple bilateral renal cysts; both kidneys enlarged (right 15.8 cm, left 16.6 cm); no hydronephrosis, no stones; liver and spleen unremarkable |
| 2022-05-23 | practice | source encounter 0: same complaints; BP 155/95 again (second occasion); flank masses and LUQ bruit again; results reviewed. **Autosomal dominant polycystic kidney disease diagnosed.** Elevated blood pressure reading recorded; home BP log asked for (R1). Nephrology referral (A3) |
| 2022-06 | practice portal | home BP log averages 126/80 (R1) |
| 2022-07-20 | outside | nephrology consult letter (A3): ADPKD confirmed by ultrasound and family history; creatinine 1.2; home readings normal, no antihypertensive; advised to avoid NSAIDs and use acetaminophen instead, stay hydrated; kidney function and urine albumin yearly; nephrology yearly |
| 2022-08-02 | practice | follow-up: headaches and palpitations have eased (R6); BP 128/80; elevated BP reading resolved |
| 2023-04-18 | practice | visit (A4): breathlessness on exertion slowly worse over the past year, using albuterol more; his older brother (37) was just diagnosed with alpha-1 antitrypsin deficiency; SpO2 95%; referred to pulmonology |
| 2023-05-30 | outside | source encounter 1, pulmonology consult letter: progressive dyspnea over a year; brother's diagnosis; breath sounds diminished on both sides, hyperresonance at the lung bases; CT chest: diffuse basilar-predominant emphysema; serum alpha-1 antitrypsin 28 mg/dL with Pi*ZZ phenotype (R7); spirometry FEV1/FVC 0.62, FEV1 71% predicted (A5). **Alpha-1 antitrypsin deficiency and emphysema diagnosed.** Plan: stop smoking; augmentation therapy not started while he smokes (A5); albuterol as needed; vaccines; yearly hepatic panel via PCP; pulmonology yearly |
| 2023-06-20 | practice | follow-up of the pulmonology letter; cessation counselling, declines medication (P); hepatic panel normal |
| 2024-04-09 | practice | visit (A6): pain in both hands and feet with walking that eases with rest; painful fingertip sores and white-then-blue fingers in the cold, over about 5 months; diminished pedal pulses; A1c, lipid panel, ESR, CRP and ANA normal (A6); referred to vascular surgery |
| 2024-05-20 | outside | source encounter 2, vascular surgery consult letter: symptoms as above (6 months); denies fever, joint pain, weight loss; no diabetes, hypertension or hyperlipidemia; smokes 1.5 packs per day for 15 years; T 36.8 C, BP 124/78, P 76, RR 14; shallow ulcers at the tips of multiple fingers and toes; no joint swelling or erythema; radial and dorsalis pedis pulses diminished bilaterally; capillary refill delayed; rest of exam unremarkable. **Thromboangiitis obliterans diagnosed** (R8). Plan: complete tobacco cessation, wound care, arteriography (A7) |
| 2024-06-06 | outside | arteriography, reported in a vascular letter (A7): segmental occlusions of the digital arteries of both hands and of the distal tibial arteries of both legs with corkscrew collaterals; proximal arteries smooth, no atherosclerosis or emboli source |
| 2025-05-29 | outside | source encounter 3, vascular surgery follow-up letter: about one year after the first visit; pain now at rest, worst at night; cold-induced pallor then blue, then redness and throbbing on rewarming; over the past month painful ulcers on the tips of two fingers and one toe; denies fever, weight loss, joint pain; smoking now 1 pack per day; exam: small shallow ulcers on the right fingertips and the left great toe; diminished hair growth on both legs; mildly decreased dorsalis pedis pulses bilaterally; radial and ulnar pulses palpable; digital Allen test abnormal in both hands (R9). Plan: complete cessation stressed again, wound care, return in a year |
| 2026-05-18 | outside | source encounter 4, ED record: 2 months of burning epigastric pain, worse several hours after meals, briefly better with eating or antacids; dark stools for a week; takes ibuprofen occasionally for muscle pain; smokes 1 pack per day; vitals normal; mild epigastric tenderness; stool guaiac positive; CBC and BMP normal (R10). GI performs upper endoscopy the same day: a 1.2 cm ulcer along the lesser curvature of the stomach, biopsied. **Gastric ulcer with hemorrhage diagnosed. Omeprazole 40 mg twice daily started; ibuprofen stopped** (A8). Discharged the next morning |
| 2026-05-21 | outside / practice | gastric biopsy pathology: chronic active gastritis with Helicobacter pylori organisms, no dysplasia or malignancy (R11). **H. pylori infection diagnosed. Bismuth quadruple therapy for 14 days started** (bismuth subsalicylate 524 mg four times daily, tetracycline 500 mg four times daily, metronidazole 500 mg three times daily, with the omeprazole) (A8) |
| 2026-05-27 | practice | post-ED visit (A8): pain much better, stools brown; BP normal; **acetaminophen 500 mg as needed started** in place of ibuprofen (A8); NSAID avoidance |
| 2026-06-04 | practice portal | course finished; **omeprazole stepped down to 40 mg once daily** (A8) |
| 2026-08-11 | outside | repeat upper endoscopy (A9): ulcer healed, a small scar at the lesser curvature; biopsies (pathology 2026-08-14): no H. pylori, no dysplasia or malignancy. **Gastric ulcer and H. pylori resolved** |
| 2026-08-17 | practice | telephone: results; **omeprazole stopped** (A9) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** BP 155/95 on two occasions in a man with ADPKD, yet the 2024 vascular letter says
  he has no hypertension, its BP is 124/78, the 2026 ED vitals are normal, and no list ever
  carries an antihypertensive. Resolution: the two office readings are carried (2022-05-09
  and 2022-05-23) and recorded as an elevated blood pressure reading, not hypertension; the
  home log and nephrology readings are normal, the reading is resolved in August 2022 and
  later office BPs run normal. No antihypertensive.
- **R2.** Family history differs across notes (encounter 0: "mother kidney disease in her
  40s"; encounter 1: "father died at 55 of alcoholic cirrhosis"). Resolution: the profile's
  version is carried (father ADPKD with ESRD; mother emphysema/COPD, former smoker; older
  brother AATD, diagnosed spring 2023). The mother's kidney disease and the father's
  cirrhosis death are not carried; the father's vital status is not stated.
- **R3.** Smoking amounts differ (22.5 pack-years; 1.5 ppd for 15 years; "15 pack-years"
  and 1 ppd; "5-year history of one pack per day"). Resolution: 1 to 1.5 ppd since about
  age 19, 22.5 pack-years stated in the 2024 vascular letter, 1.5 ppd in 2024 and 1 ppd in
  2025 and 2026 as the source gives. "5 years" and "15 pack-years" are not carried.
- **R4.** Albuterol is a home medication from encounter 0, with no lung diagnosis until 2023
  and dyspnea that began about a year before the 2023 visit. Resolution: albuterol as needed
  is started at the 2022-05-09 visit for mild exertional breathlessness, and continues on
  every list after.
- **R5.** "Mildly elevated serum aldosterone" has no number. Resolution: aldosterone
  21 ng/dL, flagged high, with a plasma renin activity of 3.4 ng/mL/h (A2) drawn with it;
  read as secondary to the cystic kidneys, no further workup.
- **R6.** The headaches and palpitations have no course in the source and appear in no later
  note. Resolution: they ease by the August 2022 visit without treatment.
- **R7.** A liver biopsy (PAS-positive, diastase-resistant periportal granules) at a
  pulmonology consult is not something that setting does. Resolution: dropped; the AATD
  diagnosis is carried by a serum level of 28 mg/dL and a Pi*ZZ phenotype in the
  pulmonology letter.
- **R8.** Encounter 2 says he "returns for thromboangiitis obliterans follow-up", while
  encounter 3 calls it his initial evaluation and the problem list dates the diagnosis to
  encounter 2. Resolution: 2024-05-20 is the first vascular visit, referred by the practice
  (A6); Buerger disease is diagnosed there. Nothing about it appears in the chart before
  the April 2024 symptoms.
- **R9.** Radial pulses diminished in 2024 and palpable in 2025: kept as written, one year
  apart (exam difference).
- **R10.** The ED gives normal vitals and a positive guaiac only. Resolution: the ED CBC
  (hemoglobin at his usual level) is normal and the BMP is normal apart from a creatinine near his slowly rising baseline; no
  transfusion. The endoscopy is done by GI during the ED stay, with an overnight
  observation and discharge on 2026-05-19.
- **R11.** "Gastric biopsy is shown below" is quiz text. Resolution: the pathology report
  shows H. pylori gastritis, matching the source's primary diagnosis.
- **R12.** The 2025 vascular note describes new ulcers over the past month, so the 2024
  ulcers healed in between. Resolution: the 2024 finger and toe ulcers heal over summer
  2024 with wound care, leaving healed scars at the tips; new ulcers appear in spring 2025.
  The 2025 ulcers stay open and dressed through autumn 2025 and are healing by May 2026.
- **Left as the source has it:** he keeps smoking throughout and never takes cessation
  medication (counselled at visits, he declines); no maintenance inhaler is added to
  albuterol; no vasoactive drug for Buerger disease; the LUQ bruit is never worked up; no
  screening kidney ultrasound before 2022 despite his father's ADPKD; he keeps taking
  ibuprofen occasionally after the nephrology advice until the 2026 bleed. No one comments
  on these.

## Additions (A): physician to confirm

- **A1.** September 2019 new-patient visit (seed: 7 years with the practice).
- **A2.** The 2022-05-09 visit that finds the first 155/95, the flank masses and bruit,
  takes an ECG (normal sinus rhythm) and orders the labs and ultrasound the source reports
  at encounter 0; plasma renin activity 3.4 ng/mL/h with the aldosterone.
- **A3.** Nephrology consult 2022-07-20 and yearly nephrology follow-up letters
  (2023-09-13, 2024-10-02, 2025-10-08): standard ongoing care for ADPKD; creatinine 1.2,
  1.25, 1.3, 1.35 at those visits; home BPs normal; no drug; NSAID avoidance advised.
- **A4.** The 2023-04-18 practice visit that refers him to pulmonology.
- **A5.** Spirometry at pulmonology: 2023 FEV1/FVC 0.62, FEV1 71% predicted; 2024-08-21
  follow-up FEV1/FVC 0.61, FEV1 70%; 2025-08-20 follow-up FEV1/FVC 0.60, FEV1 67%.
  Augmentation therapy is discussed and deferred because he still smokes. Pulmonology
  follow-up letters each year.
- **A6.** The 2024-04-09 practice visit that refers him to vascular surgery, with labs that
  exclude look-alikes: A1c 5.3%, lipids normal, ESR, CRP and ANA normal/negative.
- **A7.** One arteriogram (2024-06-06) supporting the Buerger diagnosis, findings as in the
  timeline.
- **A8.** Standard ulcer care: omeprazole 40 mg twice daily from the ED, bismuth quadruple
  therapy 14 days from 2026-05-21, ibuprofen stopped, acetaminophen 500 mg as needed
  instead, omeprazole 40 mg daily from 2026-06-04; the 2026-05-27 post-ED practice visit.
- **A9.** Repeat endoscopy 2026-08-11 for a gastric ulcer: healed, biopsies negative for
  H. pylori and malignancy; omeprazole stopped 2026-08-17.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care done there, with normal results.
- Yearly physicals (fall 2020, 2021, 2022, 2023, 2024, 2025) with vitals, BMP or CMP, CBC,
  lipids, urine albumin/creatinine from 2022, hepatic panel from 2023; creatinine 1.2 to
  about 1.35 from 2022, slowly rising; other results normal. BMI about 26.
- Smoking cessation counselling at visits (he declines medication).
- Prevention: HIV and hepatitis C screening once (nonreactive); Tdap 2019; influenza most
  falls; COVID primary series 2021 and boosters; PCV20 in 2022; PHQ-2 negative.
- Albuterol refills, portal messages about results and scheduling, a missed visit, records
  requests, work-note texture.
- Booked visit after 2026-08-27: a primary-care follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002636 on 2025-06-10. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
