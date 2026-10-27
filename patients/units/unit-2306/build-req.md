# Build requirements: unit-2306 (Michael Larkin, MRN00002306)

Source: synthetic_hospital patient 2306, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("The histology is shown below",
"A high-resolution CT of the lung is shown").

## Who he is (S)

Michael Larkin, born 1957-08-09, male, White, not Hispanic or Latino, English,
Medicare with a supplemental private plan. Lives alone in the seed's town; divorced
(texture only, no family member enters the chart through it).

- Occupation: shipyard mechanic for about 30 years, later metal fabricator; retired
  in 2018 (S: "retirement 5 years ago" at the 2023-10-20 visit).
- Tobacco: former cigarette smoker, 40 pack-years, quit in spring 2019 (S: "quit 3
  years ago" at the June 2022 visit). Tobacco use disorder in sustained remission.
- Alcohol: occasional social use, 1 to 2 drinks a week.
- Allergies: sulfa drugs (rash); shellfish (reaction not recorded).
- Surgical history: appendectomy at age 34 (1991); right knee arthroscopy at age 52
  (2010).
- Family history: father lung cancer at 72, deceased; mother type 2 diabetes and
  pancreatic cancer at 78; brother prostate cancer at 68; sister pancreatic cancer at
  58 (S, encounter 2); paternal grandfather bladder cancer.

**Joined the practice 2020-09-22** (A1, new-patient visit), age 63. Brought:

- Problems: COPD with emphysema, diagnosed 2014 at age 56 (R2); benign prostatic
  hyperplasia with lower urinary tract symptoms (hesitancy, nocturia); tobacco use
  disorder in sustained remission; nocturnal oxygen use.
- Drugs: tiotropium 18 mcg inhaled once daily; albuterol 90 mcg/actuation 2 puffs
  every 4 to 6 hours as needed for dyspnea; tamsulosin 0.4 mg at bedtime;
  acetaminophen 650 mg every 6 hours as needed for pain; supplemental oxygen 2 L/min
  by nasal cannula overnight (R3).

Height 178 cm. Weight 81 kg at joining (A1); later weights follow R6.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-22 | practice | new-patient visit (A1): history above, meds and allergies reconciled; BP normal; screening labs (P). Records requested from the prior office |
| 2022-05-20 | practice | progressive exertional dyspnea over about a year, trouble with stairs, dry cough; no fever, hemoptysis or chest pain; bibasilar crackles; SpO2 94% on room air. **Pulmonary referral** (A2) |
| 2022-06-27 | outside | pulmonary consult (enc 0): dyspnea and stair difficulty over the past year, dry cough; denies fever, hemoptysis, chest pain, recent travel. T 36.9 C (98.4 F), BP 126/78, pulse 82, RR 20, SpO2 94% on room air. Bilateral end-inspiratory basal crackles; no clubbing, cyanosis or edema. High-resolution CT chest the same day (findings in R1). PFTs: FEV1 decreased, FEV1/FVC normal, TLC reduced, DLCO reduced (A3 numbers). **Idiopathic pulmonary fibrosis diagnosed** (R1). Letter received by the practice |
| 2022-09-16 | outside | pulmonary follow-up (enc 1): dyspnea, dry cough and fatigue now about 8 months worse; shipyard mechanic 30 years; finger clubbing and bibasilar end-inspiratory crackles. HRCT (the June study, reviewed with radiology): subpleural reticulation, honeycombing and traction bronchiectasis mainly in the lower lobes; no pleural or diaphragmatic thickening, plaques or calcification. PFT: FVC and FEV1 decreased, ratio normal, DLCO decreased. **Combined pulmonary fibrosis and emphysema** named (R1). Letter received |
| 2023-02-03 | practice | epigastric pain radiating to the back for about 2 months, weight down; A1c 7.4 (A4). **CT abdomen/pelvis and gastroenterology referral**; **diabetes diagnosed, metformin 500 mg twice daily started** (A4) |
| 2023-02-14 | outside | CT abdomen and pelvis with contrast: 3 cm mass in the pancreatic head, no evidence of metastasis |
| 2023-02-24 | practice | phone: metformin raised to **1000 mg twice daily** (A4) |
| 2023-02-27 | outside | gastroenterology consult (enc 2): age 65 (R5); 3 months of epigastric pain radiating to the back, 15 lb unintentional weight loss, new-onset diabetes; no change in bowel habits, no jaundice; cachectic; abdomen soft, mild epigastric tenderness, no mass. Hemoglobin 12.1, total bilirubin 1.2, alkaline phosphatase 120, CA 19-9 560, fasting glucose 156. Coagulation studies (A5). **Cachexia** recorded. Plan EUS-guided biopsy and surgical referral |
| 2023-03-07 | outside | EUS with fine-needle aspiration (A5): **mucin-producing pancreatic adenocarcinoma**; **chronic disseminated intravascular coagulation** named with A5 labs (no bleeding or clot) |
| 2023-03-20 to 2023-03-29 | outside | admission for pancreaticoduodenectomy (A6). Post-operative **hyperchloremic metabolic acidosis due to excess normal saline infusion** (R4), resolved before discharge. Discharged on **insulin glargine 20 units at bedtime** and **pancrelipase with meals** (A6); metformin continued. **Exocrine pancreatic insufficiency** recorded (A6) |
| 2023-04-07 | practice | post-discharge visit (P); weight 68 kg |
| 2023-04-24 | outside | oncology consult (A7): adjuvant modified FOLFIRINOX, 12 cycles 2023-05-08 to 2023-10-09; CA 19-9 31 after surgery; DIC labs normalized (A5), **DIC resolved** |
| 2023-10-11 | practice | portal message and call: painless visible blood in the urine for about 2 weeks; urgent urology referral (A8) |
| 2023-10-20 | outside | urology consult (enc 3): 2 weeks of painless gross hematuria; no recent infection, no cystitis-causing drugs; metal worker, retired 5 years; BP 130/85; exam unremarkable; urinalysis gross hematuria without infection; creatinine 1.1, BUN 14. Office cystoscopy: papillary bladder tumor. **Bladder cancer** recorded (A9) |
| 2023-10-27 | outside | CT urogram (A9): papillary lesion of the left lateral bladder wall 1.7 cm, upper tracts normal |
| 2023-11-08 | outside | transurethral resection of bladder tumor (A9): low-grade noninvasive papillary urothelial carcinoma (pTa), muscularis propria present and uninvolved; single post-op intravesical gemcitabine instillation |
| 2024-02-16 | outside | urology surveillance cystoscopy negative (A9) |
| 2024-06-28 | outside | pulmonary follow-up (enc 4): daytime fatigue he puts down to poor sleep; nocturnal oxygen; walks slowly with a cane; no recent hospitalization; daily productive cough without more sputum or color change. RR 16, SpO2 94% on room air; alert, full sentences, no distress; decreased breath sounds both sides, mild wheeze; no edema or JVD. ABG drawn as routine follow-up (R4). Chest x-ray: flattened diaphragms, increased retrosternal air space. Letter received |
| 2024-11-22 | outside | urology surveillance cystoscopy negative (A9) |
| 2024-12-11 | practice | months of worsening weakness on stairs and rising from chairs; weight down; labs (P, normal CK and TSH); pulmonary visit moved up (A10) |
| 2024-12-27 | outside | pulmonary visit (enc 5): progressive proximal weakness, better with repeated effort; persistent cough; 6.8 kg (15 lb) unintentional weight loss; lower-extremity reflexes diminished, improving after exercise. Chest x-ray: right hilar mass. **Suspected lung malignancy**; bronchoscopic biopsy arranged (R7) |
| 2025-01-08 | outside | EBUS bronchoscopy biopsy (R7): **small cell lung carcinoma**; staging limited (A11); P/Q-type calcium channel antibodies positive: **Lambert-Eaton myasthenic syndrome** (R7) |
| 2025-01-20 to 2025-03-24 | outside | oncology (A11): carboplatin and etoposide, 4 cycles; thoracic radiation not given because of IPF |
| 2025-05-12 | outside | restaging CT chest (A11): partial response; residual central right hilar mass 2.7 cm (4.3 cm in January) with hilar lymphadenopathy |
| 2025-06-06 | practice | routine visit: sodium 127 (A12); **fluid restriction 1.2 L/day** |
| 2025-06-13 | practice | lab only: sodium 125 (A12); phone, restriction tightened to 1 L/day |
| 2025-06-18 to 2025-06-21 | outside | ED (enc 6): confusion, fatigue and nausea over the past week; sodium stays low despite fluid restriction; mild confusion; euvolemic, no edema, no orthostatic hypotension, no JVD. Sodium 120, serum osmolality 260, urine osmolality 500; urine sodium 58 (A13). Recent chest CT: central lung mass with hilar lymphadenopathy (the May study). **SIADH** diagnosed. Admitted 3 days (A13): fluid restriction, **sodium chloride 1 g three times daily started**; sodium 126 at discharge, confusion cleared |
| 2025-06-27 | practice | post-discharge visit (A13): sodium 130 |
| 2025-12-19 | practice | **last practice visit**: routine follow-up; sodium 133, A1c 7.0 (A13, A4); next visit booked |
| 2026-04-27 | outside | urology (enc 7): worsening back pain and weight loss; hesitancy and nocturia; firm nodular prostate; PSA markedly elevated (64 ng/mL, A14); bone scan multiple areas of uptake in lumbar spine and pelvis; biopsy adenocarcinoma (A14); **leuprolide started** with bicalutamide for flare (A14). Records filed only; the practice lists are not updated (after last seen) |
| 2026-09-15 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 stamps idiopathic pulmonary fibrosis to that day but shows its CT
  only as "is shown"; encounter 1 gives the CT findings. Resolution: one HRCT,
  performed 2022-06-27; its report carries encounter 1's findings plus upper-lobe
  centrilobular and paraseptal emphysema (the emphysema the source names elsewhere,
  A3). IPF is diagnosed 2022-06-27; combined pulmonary fibrosis and emphysema is the
  overall label from 2022-09-16. The profile lists IPF and CPFE from the start: not
  carried before June 2022. The practice's list keeps COPD/emphysema, IPF and CPFE.
  Clubbing absent in June and present in September: kept as the two letters say.
- **R2.** Emphysema "diagnosed age 56" (profile) and "8 years ago" (2024 note). Resolution:
  2014, at age 56. "8 years ago" not carried.
- **R3.** The profile lists every home drug at every encounter. Resolution: tiotropium,
  albuterol, tamsulosin, acetaminophen and nocturnal oxygen are what he brought in
  2020. Metformin starts 2023-02-03 (A4); insulin glargine starts 2023-03-29 (A6).
  Outside letters may copy the full source list, as outside lists do; practice lists
  follow this course.
- **R4.** The source stamps hyperchloremic metabolic acidosis from excess normal saline
  to the June 2024 pulmonary visit, whose own note records no hospital stay or
  infusion and only says an ABG was drawn. Resolution: the acidosis occurred after
  the March 2023 pancreaticoduodenectomy (large saline volumes), shown on post-op
  chemistry (chloride 114, bicarbonate 17, anion gap 8, A6) and resolved before
  discharge by switching fluids. The June 2024 ABG is reported in the letter without
  numbers as showing no acid-base disturbance.
- **R5.** Encounter 2 says 64 years old; on 2023-02-27 he is 65. Ages follow the birth
  date. "January and April 2020" pulmonary visits become June and September 2022.
- **R6.** Weight: 81 kg at joining, 79 by May 2022, about 72 at the gastroenterology
  visit (15 lb loss), 68 after surgery, 67 on chemotherapy, recovering to about 71 by
  mid-2024, about 64 in December 2024 (6.8 kg loss), 61 to 62 through 2025 chemotherapy
  and after. Cachexia stays on the list. Hemoglobin runs about 11 to 13 from the 2023 diagnosis on (cancer and chemotherapy), with no anemia diagnosis or treatment. Resting SpO2 about 95% at joining and 93 to 95% from 2022.
- **R7.** The pulmonary note says "a biopsy is obtained" and shows the histology as a
  quiz image; the ED note and later lists still say "suspected." Resolution: the biopsy
  (EBUS, 2025-01-08) shows small cell carcinoma, the specific form of the suspected
  lung malignancy; the weakness that improves with effort and the reflexes that
  return after exercise are named Lambert-Eaton myasthenic syndrome. Outside letters
  may still carry "suspected" on copied lists.
- **R8.** The source's ED and urology problem lists carry pancreatic cancer "with chronic
  DIC" forward. Resolution: DIC resolved after resection (A5, A7); pancreatic
  adenocarcinoma stays on the list as treated, under surveillance. Bladder cancer stays
  on the list under surveillance.
- **R9.** Family history: mother with pancreatic cancer at 78 (profile) and a sister at
  58 (encounter 2). Both are carried.
- **Left as the source has it:** no antifibrotic is ever started for IPF and lung
  transplant is never raised; no statin for a diabetic over 65; no germline genetic
  testing despite the family history; no PSA screening by the practice. No note
  comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2020-09-22 new-patient visit (the first source encounter reads as established
  care with a long problem list). Weight 81 kg, height 178 cm.
- **A2.** 2022-05-20 practice visit that sends him to pulmonology.
- **A3.** PFT numbers (2022-06-27): FVC 66% predicted, FEV1 68% predicted, FEV1/FVC
  0.80, TLC 70% predicted, DLCO 38% predicted. HRCT emphysema descriptors (R1).
- **A4.** 2023-02-03 practice visit for the epigastric pain: A1c 7.4, which with the
  gastroenterology fasting glucose makes the new-onset diabetes; metformin 500 mg
  twice daily, then 1000 mg twice daily from 2023-02-24 (the source dose). A1c about 7
  on treatment after surgery (6.4 to 7.4).
- **A5.** Source names chronic DIC with no evidence. Labs 2023-02-27: platelets 118,
  PT 14.4 s, INR 1.2, fibrinogen 142 mg/dL, D-dimer 5.6 mcg/mL FEU; no bleeding or
  thrombosis. Normalized after resection (2023-04-24: platelets 236, fibrinogen 288,
  D-dimer 0.7). EUS-FNA 2023-03-07 confirms mucin-producing adenocarcinoma.
- **A6.** Treatment of the localized pancreatic head cancer: pancreaticoduodenectomy
  2023-03-21 (admitted 2023-03-20 to 2023-03-29); pathology 3.0 cm mucin-producing
  adenocarcinoma, margins negative, 0 of 16 nodes (pT2 N0). Post-op acidosis per R4.
  Insulin glargine 20 units at bedtime (the source dose) and pancrelipase (lipase
  24,000 units) 2 capsules with meals and 1 with snacks for the pancreatic insufficiency
  the source names; exocrine pancreatic insufficiency recorded.
- **A7.** Oncology: chest port placed for chemotherapy (removed 2024-10-29); adjuvant modified FOLFIRINOX, 12 cycles, 2023-05-08 to 2023-10-09,
  then surveillance CT and CA 19-9 every 3 to 6 months, no recurrence through the
  chart (CA 19-9 under 37 on each later check).
- **A8.** 2023-10-11 practice call that sends him to urology.
- **A9.** Bladder cancer course: office cystoscopy 2023-10-20 (papillary tumor); CT
  urogram 2023-10-27 (1.7 cm left lateral wall lesion, upper tracts normal); TURBT
  2023-11-08, low-grade pTa urothelial carcinoma, single gemcitabine instillation;
  surveillance cystoscopy negative 2024-02-16, 2024-11-22 and 2025-11-14.
- **A10.** 2024-12-11 practice visit for the weakness and weight loss.
- **A11.** Lung cancer course: EBUS biopsy 2025-01-08, small cell carcinoma; PET-CT
  limited stage (right hilar mass 4.3 cm, ipsilateral hilar and subcarinal nodes, no
  distant disease); brain MRI without metastasis; P/Q-type voltage-gated calcium
  channel antibodies positive. Carboplatin and etoposide, 4 cycles 2025-01-20 to
  2025-03-24; thoracic radiation declined by radiation oncology because of IPF.
  Restaging CT 2025-05-12: partial response, residual mass 2.7 cm with hilar nodes.
  Later CTs (2025-09-15, 2026-01-12) stable. Weakness improves after chemotherapy.
- **A12.** Sodium 127 on 2025-06-06 and 125 on 2025-06-13 (the source's "despite fluid
  restriction" needs a restriction to start somewhere); fluid restriction 1.2 L then
  1 L a day.
- **A13.** ED and admission 2025-06-18 to 2025-06-21: urine sodium 58, TSH 2.1, AM
  cortisol 15; sodium chloride 1 g three times daily; sodium 126 at discharge.
  Practice sodium 130 (2025-06-27), 132 (2025-08-12), 131 (2025-10-07), 133 (2025-12-19).
- **A14.** Prostate cancer course at urology (enc 7, collapsed in the source): visit
  2026-04-27 with PSA 64 ng/mL; bone scan 2026-05-04; transrectal biopsy 2026-05-11,
  adenocarcinoma, ISUP grade group 4; bicalutamide 50 mg daily from 2026-05-26 for 4
  weeks and leuprolide 22.5 mg every 3 months from 2026-06-02. Letters reach the
  practice; lists not updated.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice: colonoscopy 2018 normal; prior lung care on
  tiotropium and overnight oxygen.
- Visits every 3 to 6 months for lungs, diabetes (from 2023) and cancer coordination;
  Medicare wellness visits yearly; A1c, BMP or CMP, CBC as the conditions call for;
  urine albumin/creatinine yearly from 2023; diabetic eye exam letters without
  retinopathy; foot exams. One lipid panel at joining (LDL 104), not repeated.
- Prevention: influenza each fall (high-dose from age 65), COVID vaccines, PCV20,
  Shingrix series, RSV (Arexvy, fall 2023), Tdap; hepatitis C and HIV screening once
  (negative); PHQ-2/PHQ-9 screens (low scores).
- Refills, pharmacy and prior-authorization traffic (insulin pens, pancrelipase),
  oxygen supplier paperwork, portal messages about results, refills and scheduling,
  a cancelled visit.
- After 2025-12-19: portal messages, refills, outside records filed as received.
- Booked visit after 2026-08-27: routine follow-up on 2026-09-15.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002306 on 2025-12-19. Practice contacts after that day are
limited to messages, refills and filed records; outside care after it still happens, and
its records are filed as received without changing the practice's lists. A visit is
booked after today.
