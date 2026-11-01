# Build requirements: unit-2561 (Jacob Kendall, MRN00002561)

Source: synthetic_hospital patient 2561, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, the "slit lamp examination is shown below" image or
the pulmonology quiz about lung physiology and gas exchange.

## Who he is (S)

Jacob Kendall, born 1997-10-16, male, White, not Hispanic or Latino, English, lives in
Belchertown (01007), employer-sponsored PPO (Blue Cross Blue Shield). Software engineer
(starts a new job in spring 2023, which brings the employee health screening of
encounter 3). Never smoker, no recreational drugs, social alcohol (weekends, 2-3
drinks). Exercises regularly. Single. Allergies: no known drug allergies, throughout.

Family history (R3, R4): father died at 45 of liver failure attributed to long-term
alcohol use, and had alpha-1 antitrypsin deficiency (PiZZ) with emphysema; mother had
unexplained facial swelling episodes in her youth and is diagnosed with hereditary
angioedema type I in 2022; a maternal uncle had the same facial swelling in his youth;
mother and that maternal uncle have skin findings like the patient's (NF1-type spots),
father did not; maternal grandfather hypertension.

Surgical history is built by the timeline: right radical inguinal orchiectomy
(2023-07-06), posterior spinal fusion with decompressive laminectomy (2024-12-08), left
VATS evacuation of pulmonary hematoma (2026-06-21). Before 2023 he has had no surgery.

**Joins the practice 2019-09-10** (A1), age 21: healthy, no problems, no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new patient visit (A1). Healthy; family history as known then: father's death at 45 from alcohol-related liver failure, maternal grandfather hypertension. Height 180 cm, weight 64 kg. Screening labs normal (P) |
| 2021-06-18 | practice | annual physical, age 23 (enc 0). Exam: multiple flat hyperpigmented macules on trunk and limbs, clusters of small brown freckle-like lesions in both axillae and the groin; no subcutaneous nodules, no oral mucosal lesions. He says his mother and a maternal uncle have similar spots, his father did not. **Neurofibromatosis type 1 diagnosed** clinically. Ophthalmology referral for a slit-lamp exam (R1). He also mentions about a year of swelling episodes (A2): referral to allergy |
| 2021-07-20 | outside | ophthalmology letter (R1): dilated and slit-lamp exam normal, no Lisch nodules, no optic nerve findings; recheck yearly |
| 2021-08-23 | outside | allergy and immunology consult (enc 1): about a year of episodes of swelling of lips, hands and genitals, each building over hours and gone on its own in 2 to 3 days; three earlier episodes of severe cramping abdominal pain with bloating and watery diarrhea, afebrile, no blood in stool; no rash, wheeze or itch; no food allergy, no new drugs. Mother and maternal uncle had unexplained facial swelling in their youth. No daily medications. Vitals stable; exam unremarkable. Labs (A3): C4 6 mg/dL (ref 14-40), C1 esterase inhibitor antigen 7 mg/dL (ref 21-39), C1 esterase inhibitor function 22% (ref >67%). **Hereditary angioedema type I diagnosed; icatibant 30 mg subcutaneous as needed for attacks started** (A4). Avoid ACE inhibitors and estrogens; family testing advised; yearly allergy follow-up |
| 2022-05-10 | practice | office visit (A5): exertional breathlessness slowly worse for about 2 years, now winded after one flight of stairs; no wheeze, chest pain or cough. SpO2 93%. Referred to pulmonology |
| 2022-06-20 | outside | pulmonology consult, age 24 (enc 2, first pulmonology visit, R2). Same history; no travel; never smoked or used drugs. SpO2 92% room air. Thin, mildly tachypneic at rest; decreased breath sounds and prolonged expiration without wheezes or crackles, mild bilateral hyperresonance, slightly increased AP diameter; no clubbing; heart and abdomen normal; no leg edema. Chest x-ray: bilateral hyperlucency, several large bullae in the lower lobes. Plan: further testing and genetic counseling referral; he is worried about his long-term outlook |
| 2022-06-20 to 2022-07-06 | outside | workup (A6): alpha-1 antitrypsin level 22 mg/dL (ref 100-190), phenotype PiZZ. Pulmonary function tests: FEV1 1.92 L (45% predicted), FVC 4.10 L (79% predicted), FEV1/FVC 0.47, no significant bronchodilator response, DLCO 48% predicted. CT chest: panlobular emphysema, lower-lobe predominant, several large bullae in both lower lobes, the largest 6.1 cm in the left lower lobe; no nodules |
| 2022-07-25 | outside | pulmonology follow-up letter (A6): **alpha-1 antitrypsin deficiency, PiZZ, with early-onset panacinar (panlobular) emphysema diagnosed.** **Started:** weekly IV alpha-1 proteinase inhibitor augmentation 60 mg/kg through home infusion (first infusion 2022-08-04), **tiotropium 2.5 mcg 2 inhalations daily, albuterol HFA 2 puffs every 4-6 h as needed** (A7). Asks the practice for hepatitis A and B and pneumococcal vaccines and yearly liver tests and liver ultrasound |
| 2022-08-15 | outside | genetic counseling letter (A6, R3): autosomal recessive inheritance explained; relatives confirm the father had been diagnosed with PiZZ alpha-1 antitrypsin deficiency and emphysema before his death; mother offered carrier testing |
| 2022-08-30 | practice | follow-up (P): reviews the diagnosis; mother has since been diagnosed with hereditary angioedema type I (R4). Hepatic panel normal; liver ultrasound ordered (normal, 2022-09-13); PCV20 and the first hepatitis A and B dose |
| 2023-06-07 | outside | employer new-hire screening labs (enc 3): calcium 12.1 mg/dL (ref 8.6-10.3), creatinine normal (0.9 mg/dL, A8); copy brought to the practice |
| 2023-06-19 | practice | office visit (enc 3): follow-up of NF1, HAE and AATD; feels well, no symptoms; exercises regularly; denies polyuria, polydipsia, kidney stones or fractures. Repeat calcium, PTH, PTHrP, vitamin D ordered (A8) |
| 2023-06-21 | practice | results (A8): calcium 12.3 mg/dL, PTH 8 pg/mL (ref 15-65), PTHrP 4.8 pmol/L (ref <2.0), 25-OH vitamin D 32 ng/mL. **Hypercalcemia (PTH-independent, PTHrP-mediated) diagnosed**; CT chest, abdomen and pelvis with contrast ordered; oral fluids encouraged |
| 2023-06-23 | outside | CT chest, abdomen and pelvis (A9): 3.6 cm interaortocaval lymph node; no other lymphadenopathy; liver, kidneys, adrenals normal; lungs show the known lower-lobe emphysema and bullae, no nodules |
| 2023-06-26 | practice | office visit (A9): firm 2-3 cm nodule in the right testis on exam; scrotal ultrasound same day: 2.6 cm heterogeneous hypoechoic intratesticular mass, right; left testis normal. Tumor markers: AFP 486 ng/mL, beta-hCG 2,140 mIU/mL, LDH 298 U/L (ref 120-246). Urgent urology referral |
| 2023-07-06 | outside | **right radical inguinal orchiectomy** (A9); pathology: mixed nonseminomatous germ cell tumor (embryonal carcinoma and yolk sac tumor), 2.8 cm, confined to the testis, no lymphovascular invasion, spermatic cord margin negative. **Testicular nonseminomatous germ cell tumor, stage IIB, with paraneoplastic (malignancy-associated) hypercalcemia.** Post-op markers 2023-07-13: AFP 210, beta-hCG 380, calcium 11.6 |
| 2023-07-17 | outside | oncology consult (A9): stage IIB, S1, good risk. **Four cycles of etoposide and cisplatin (EP); bleomycin omitted because of the emphysema.** Cycles start 2023-07-24, 08-14, 09-04, 09-25. **Ondansetron 8 mg as needed for nausea** with chemotherapy (short) |
| 2023-08-14 | outside | calcium 9.6: **hypercalcemia resolved** |
| 2023-10-16 | outside | end-of-treatment markers normal: AFP 4 ng/mL, beta-hCG <2 mIU/mL, LDH normal |
| 2023-11-06 | outside | CT chest, abdomen and pelvis: interaortocaval node down to 0.9 cm; no new disease. Complete response; surveillance (markers every 2-3 months the first year, then every 4-6 months; CT abdomen and pelvis 2024-05, 2024-11, 2025-11, all stable with the node 0.8-0.9 cm) |
| 2023-12-05 | practice | follow-up (P): testicular cancer moved to history of testicular cancer |
| 2024-12-07 | outside | motorcycle crash (enc 4, Saturday): no helmet, thrown and landed on his back. On arrival BP 85/50, HR 48, RR 22, temp 35.6 C, shallow breathing, SpO2 92% room air. Flaccid paralysis and loss of sensation below the nipple line; distended bladder, indwelling catheter placed, 800 mL drained. CT spine: T4 vertebral fracture with spinal cord injury. **Neurogenic (distributive) shock; hypoxemia.** Admitted to ICU. Course (A10, R5): norepinephrine and atropine; **posterior instrumented fusion T2-T6 with T4 decompressive laminectomy 2024-12-08**, with perioperative C1 esterase inhibitor concentrate for the HAE (A11); pressors off 2024-12-11; room air by 2024-12-11; after spinal shock settled, sacral sensation and partial motor function below the level: incomplete T4 spinal cord injury; to inpatient rehabilitation 2024-12-18 |
| 2025-01-31 | outside | rehab discharge (A10): walking with a walker, voiding on his own (catheter out 2024-12-30), oxycodone-acetaminophen 5/325 short taper finished at discharge; outpatient physical therapy |
| 2025-02-11 | practice | post-rehab visit (P): shock and hypoxemia resolved on the list; T4 fracture and incomplete thoracic cord injury on the list |
| 2025-06 to 2025-09 | outside | physical medicine and rehabilitation and neurosurgery letters (A10): walking independently without a device by June 2025, mild residual weakness of both hip flexors, cleared to drive June 2025; fusion solid on thoracic spine x-ray 2025-09; fracture healed |
| 2026-06-19 | outside | ED (enc 5, Friday): restrained driver in a high-speed car crash; alert, anxious, short of breath. BP 130/85, HR 110, RR 28, SpO2 89% room air. No outward chest wall injury; decreased breath sounds on the left. ABG on room air: pH 7.45, PaCO2 32 mmHg, PaO2 60 mmHg, HCO3 24 mEq/L. Chest x-ray: patchy, irregular alveolar opacities in the left lung, no rib fractures. **Left pulmonary contusion with hypoxemia.** Admitted (A12) |
| 2026-06-19 to 2026-06-26 | outside | admission (A12, R6): CT chest: left lower lobe contusion with a 4.8 cm intraparenchymal hematoma, no pneumothorax, no rib fractures, spinal hardware intact; oxygen 3 L nasal cannula. 2026-06-21 more oxygen needed and repeat CT shows the hematoma 6.3 cm: **left VATS evacuation of pulmonary hematoma** with C1 esterase inhibitor prophylaxis (A11). Chest tube out 2026-06-24; room air 2026-06-25, SpO2 92%; discharged 2026-06-26 on oxycodone 5 mg every 6 h as needed for 5 days and acetaminophen; augmentation infusions resume the following week |
| 2026-07-10 | practice | **last seen**: post-hospital visit; SpO2 92%; slightly decreased breath sounds at the left base, port sites healing; CBC and BMP drawn (resulted same day, normal) |
| 2026-07-28 | outside | thoracic surgery follow-up letter (A12): chest x-ray with a small residual left lower opacity, improving; back to desk work |
| 2026-08-17 | outside | pulmonology follow-up letter (P): back on weekly augmentation, tiotropium and albuterol as needed; recheck PFTs in 3 months. Filed; the practice lists are not changed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 says a slit-lamp exam "is shown below" with no result. Resolution:
  the practice refers him to ophthalmology; the letter reports a normal slit-lamp exam
  without Lisch nodules. NF1 rests on the skin findings and the affected mother.
- **R2.** The profile lists all four chronic conditions and all three operations from
  the first encounter, and the problem lists stamp each diagnosis at a later encounter.
  Resolution: each condition starts at its encounter (NF1 2021-06, HAE 2021-08, AATD
  2022-06/07, testicular cancer and its hypercalcemia 2023-06/07), and each operation
  follows the event that calls for it. Encounter 2 says he "returns" to pulmonology and
  already names AATD; resolution: it is his first pulmonology visit and AATD is
  confirmed by the workup after it. Encounter 1 calls the HAE "known"; it is diagnosed
  at that visit.
- **R3.** The father appears three ways: AATD PiZZ with emphysema (profile), death at 45
  from alcohol-related liver failure (encounter 2), and "mildly high calcium" (encounter 3,
  present tense). Resolution: he died at 45 of liver failure attributed to alcohol and
  had PiZZ AATD with emphysema (the patient learns the AATD diagnosis through genetic
  counseling in 2022). The calcium remark is not carried.
- **R4.** Family history of skin findings and swelling: the profile names a paternal uncle
  with NF1 and a mother with HAE; encounter 0 says mother and maternal uncle have similar
  skin findings and father does not; encounter 1 says mother and maternal uncle had
  unexplained facial swelling in their youth. Resolution: mother and the maternal uncle
  carry both the skin findings and the swelling history; the mother is diagnosed with HAE
  type I in 2022 after the patient. The paternal uncle with NF1 is not carried. The
  comorbidity "family history of hereditary hemorrhagic telangiectasia" names no
  relative and is not carried.
- **R5.** Encounter 3 is a familial hypocalciuric hypercalcemia vignette (genetic test
  confirming a calcium-homeostasis mutation) while the source's diagnosis is
  malignancy-associated hypercalcemia from a testicular germ cell tumor. Resolution: the
  genetic test is not carried; the hypercalcemia is PTHrP-mediated and leads to the
  tumor (A8, A9).
- **R6.** The surgical history says fusion T5-T9 while the CT shows the fracture at T4.
  Resolution: the fracture is at T4 (as imaged) and the fusion spans T2-T6 with a T4
  laminectomy. The surgical history adds "rib fixation" to the VATS while the chest
  x-ray shows no rib fractures and no chest wall injury; resolution: no rib fixation,
  VATS evacuation of the pulmonary hematoma only. "Distributive shock (neurogenic or
  septic)" is neurogenic.
- **R7.** Encounter 5 says "He takes no medications" while he is on augmentation and
  inhalers (A6, A7). Resolution: the ED note's list is not carried; the admission
  records list his drugs.
- **R8.** The source's flaccid paralysis below the nipple line is the arrival exam in
  spinal shock; with the 2026 crash as a driver, the injury is incomplete and recovers
  (A10).
- **Left as the source has it:** HAE stays on as-needed treatment only, without long-term
  prophylaxis; he keeps riding and driving after the injuries; nobody comments on the
  motorcycle helmet beyond the ED record.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (the source starts with a routine exam; the seed
  gives 7 years with the practice).
- **A2.** At the 2021-06-18 physical he mentions the swelling episodes, which makes the
  allergy referral (encounter 1 follows "a routine internal medicine visit two months ago").
- **A3.** Complement labs to support HAE type I: C4 6, C1-INH antigen 7 mg/dL, function 22%.
- **A4.** Icatibant 30 mg subcutaneous as needed for attacks (standard on-demand therapy),
  prescribed by allergy.
- **A5.** The 2022-05-10 practice visit that sends him to pulmonology.
- **A6.** AATD workup and diagnosis: AAT 22 mg/dL, PiZZ; PFTs FEV1 1.92 L (45%), FVC
  4.10 L (79%), ratio 0.47, DLCO 48%; CT as in the timeline; genetic counseling;
  weekly augmentation therapy 60 mg/kg (standard for PiZZ with emphysema and this FEV1).
- **A7.** Tiotropium daily and albuterol as needed for symptomatic emphysema.
- **A8.** Hypercalcemia workup: calcium 12.1 then 12.3, creatinine 0.9, PTH 8, PTHrP 4.8,
  vitamin D 32.
- **A9.** Testicular cancer course: CT with a 3.6 cm interaortocaval node, testicular
  nodule on exam, scrotal ultrasound 2.6 cm right mass, AFP 486, beta-hCG 2,140, LDH 298;
  orchiectomy and pathology as listed; post-op AFP 210, beta-hCG 380, calcium 11.6;
  EP x4; calcium 9.6 on 2023-08-14; normal markers (AFP 4, hCG <2) 2023-10-16; node
  0.9 cm 2023-11-06, then surveillance, all stable without recurrence; surveillance
  markers run AFP 3-5 ng/mL, beta-hCG <2 mIU/mL, LDH 150-220 U/L.
- **A10.** Spinal injury course: blood cultures on arrival with no growth (septic shock
  excluded), head CT normal, ICU pressors and atropine, fusion T2-T6 with T4
  laminectomy, incomplete injury, rehab to walker, catheter out 2024-12-30, short
  oxycodone-acetaminophen, independent walking with mild hip flexor weakness by June 2025,
  driving clearance June 2025, solid fusion September 2025.
- **A11.** Perioperative C1 esterase inhibitor concentrate before each operation for HAE.
- **A12.** 2026 admission: CT with 4.8 cm then 6.3 cm hematoma, oxygen 3 L, VATS on
  2026-06-21, chest tube out 06-24, room air 06-25 (SpO2 92%), discharge 06-26 with
  5 days of oxycodone 5 mg as needed and acetaminophen; thoracic surgery follow-up
  2026-07-28 with an improving small residual opacity.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries (how a size was measured, a procedure's access
  site), but nothing that adds a territory, a severity grade or a symptom, or changes the
  diagnosis or plan.
- Yearly physicals with CBC, CMP and a lipid panel once or twice; yearly hepatic panel for
  AATD; PHQ-2/PHQ-9 and GAD-7 screens with normal or low scores; HIV and hepatitis C
  screening once (2019, negative); Tdap 2019; influenza each fall; COVID vaccines
  (original series spring 2021 at a pharmacy, then yearly updated doses); PCV20 2022;
  hepatitis A and B series 2022-2023.
- NF1: yearly skin and blood pressure checks (normal BP), ophthalmology letters yearly
  through 2023, normal.
- HAE: yearly allergy letters; icatibant used for an occasional attack, refills and a prior
  authorization; no change in treatment.
- AATD: pulmonology letters every 6 to 12 months, stable; resting SpO2 at practice
  visits 92-94% on room air from 2022; home infusion and specialty
  pharmacy paperwork; weight runs thin (62-64 kg; 59 kg at the end of chemotherapy, 60 kg
  at rehab discharge, back to 62-63 kg).
- Oncology surveillance letters and marker results, all normal after October 2023.
- Physical therapy notes in 2025; neurosurgery and PM&R letters as in the timeline.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills, work notes and scheduling, a missed or cancelled visit.
- No visit is booked after 2026-08-27 (seed booked false).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002561 on 2026-07-10. Practice contacts after that day are
only messages and refills; outside care after it still happens, and its records are
filed as received without changing the practice's lists. Nothing is booked.
