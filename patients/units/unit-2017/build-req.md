# Build requirements: unit-2017 (Sean Hennessey, MRN00002017)

Source: synthetic_hospital patient 2017, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its quiz text (the observational stroke study framed
as a question, the case-control study design, the unfinished "most likely to compress"
sentence).

## Who he is (S)

Born 1966-12-02, male, White, not Hispanic, English-speaking, lives in Easthampton
with his wife. Medicare (disability) with private supplemental coverage. Former
construction foreman, on medical leave throughout his time with the practice.
Alcohol: occasional, 1 to 2 drinks a month. Allergies: none known (NKDA).

Tobacco: cigarettes since his teens, about 2 packs a day, **80 pack-years** (R1);
current smoker on joining; **quits in early October 2022** and stays quit.

Surgical history: inguinal hernia repair at age 45 (2012, before joining).
Left femoral artery stent in May 2021 (S "2 years ago" at the May 2023 cardiology
visit; R2).

Family history: father lung cancer, died at 60; mother hypertension, living (80 in
2023); paternal uncle colon cancer.

**Joined the practice 2020-09-21** (A1), age 53, bringing: essential hypertension
since about 2011, on hydrochlorothiazide 25 mg daily and well controlled;
hyperlipidemia on atorvastatin 40 mg daily; peripheral arterial disease (calf
claudication, diagnosed at his prior practice) on aspirin 81 mg daily. Not yet
diagnosed with lung disease.

## Records texture

Chart with gaps (seed). The 2021 vascular stent never sends a note: the practice
learns of it from the patient. The October 2022 pulmonology letter arrives about six
weeks late; the practice first learns of the inhalers from a pharmacy fill. The 2024
rigid bronchoscopy operative note never arrives (the pathology report does). The
November 2025 discharge summary arrives two weeks late, after the patient's wife
calls. The January 2026 aneurysm repair operative note never arrives (the vascular
follow-up letter does). Outside letters copy stale medication lists (lisinopril
missing on some, as the source has it). The practice medication list lags by one
visit after the durvalumab start and the docetaxel switch.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-21 | practice | **new patient visit** (A1): history as above; BP normal on HCTZ; height 180 cm, weight 92 kg; CBC, CMP, lipids, hepatitis C and HIV screens (P, normal; Cr 1.0); smoking cessation advised, not ready; declines colon cancer screening (gap, see below); influenza vaccine |
| 2021-03-22 | practice | annual wellness visit; left calf claudication now at about one block; **vascular surgery referral** (A2) |
| 2021-05 | outside | **left femoral artery stent** by vascular surgery (R2); no note ever reaches the practice |
| 2021-09-27 | practice | follow-up; reports the stent in May, walks farther; records requested, never received; influenza vaccine |
| 2022-03-28 | practice | annual wellness visit; mentions getting winded with exertion over recent months, thinks it is age; still smoking |
| 2022-09-19 | practice | worse dyspnea over the past year: can't walk several blocks without stopping; has been using his wife's albuterol inhaler now and then; no cough or sputum; **pulmonology referral and pulmonary function testing ordered** (A3); influenza vaccine |
| 2022-10 (early) | — | **quits smoking** (patient report by phone 2022-10-10) |
| 2022-10-24 | outside | pulmonary function tests at the pulmonary lab: **FEV1/FVC 55%; total lung capacity and residual volume both elevated** |
| 2022-10-31 | outside | pulmonology consult: gradually worsening exertional dyspnea for a year; no cough or sputum; uses his wife's albuterol; 80 pack-years, quit within the last month. Exam: decreased breath sounds both sides, hyperresonance, increased AP diameter, accessory muscle use at rest, loud P2, mild peripheral edema. **Severe upper lobe predominant emphysema (COPD) diagnosed. Tiotropium 2.5 mcg 2 inhalations daily and albuterol HFA as needed started** (A4). Letter received 2022-12-12; pharmacy fill 2022-11-07 |
| 2023-01-23 | practice | COPD and HTN follow-up; still not smoking; **BP 184/96**, home readings similar; recheck with home log (P) |
| 2023-02-27 | practice | home systolic readings 180 to 190 for the past few months; **lisinopril 20 mg daily started** (R3), HCTZ continued; BMP ordered in 2 weeks (he never goes); **cardiology referral** for worsening hypertension (A5) |
| 2023-05-11 | outside | cardiology consult: 12-year history of hypertension, well controlled on HCTZ until the past 6 months, systolic 180 to 190, second agent recently added. Femoral artery stent 2 years ago. Denies headaches, palpitations, muscle weakness. **Creatinine 1.7 mg/dL (baseline 1.0); plasma renin activity markedly elevated.** **Renal artery stenosis diagnosed**; renal artery duplex ordered (A6); continue current regimen. Letter received 2023-05-22 |
| 2023-05-25 | outside | renal artery duplex (A6): **right renal artery stenosis of 60% or greater; left renal artery without significant stenosis**; kidneys normal in size. Cardiology note received 2023-06-05: atherosclerotic, medical management, continue lisinopril and HCTZ, recheck creatinine |
| 2023-06-12 | practice | lab only: BMP, **creatinine 1.4** (A6) |
| 2023-08-21 | practice | follow-up; BP 146/88; **creatinine 1.2**; started a community plant-based eating and exercise program in May (P) |
| 2023-10-16 | practice | nurse visit: influenza, PCV20, COVID updated vaccines (P) |
| 2023-11-17 | practice | routine follow-up: no complaints; plant-based diet and regular exercise program; 10-pound weight loss over 6 months (weight 87.5 kg); asks about an observational study linking such a program to fewer strokes and doubts it; the PCP explains what an observational study can and can't show (no trial figures). **BP 138/86; BMI 27; total cholesterol 190, HDL 40, LDL 120, triglycerides 150, fasting glucose 98; creatinine 1.1** (R4) |
| 2024-02 (mid) | — | hoarseness, cough and worse dyspnea begin |
| 2024-04-01 | practice | hoarseness and cough for about 7 weeks, more short of breath; no fever, weight loss or night sweats; chest x-ray (A7): hyperinflation consistent with emphysema, no mass or infiltrate; **expedited pulmonology referral** |
| 2024-05-10 | outside | pulmonology: 3 months of progressive dyspnea, hoarseness and cough; no fever, weight loss, night sweats. Exam: **stridor in both inspiration and expiration, raspy voice, no palpable lymphadenopathy. Flow-volume loop with flattening of both inspiratory and expiratory limbs.** Fixed central airway obstruction; **tracheal neoplasm**; CT chest ordered (A8). Letter received 2024-05-20 |
| 2024-05-14 | outside | CT chest (A8): **polypoid intraluminal mass of the mid trachea, about 1.8 cm, narrowing the lumen by about 50%**; upper lobe predominant emphysema; no lung mass, no enlarged lymph nodes |
| 2024-06-03 | outside | rigid bronchoscopy with excision of the tracheal mass (A8); operative note never received. Pathology report received 2024-06-10: **squamous papilloma, no dysplasia or malignancy** |
| 2024-06-17 | practice | post-procedure follow-up: voice nearly normal, no stridor, breathing back to his baseline |
| 2024-10-21 | practice | about 3 months of progressive fatigue and exertional dyspnea, 10-pound unintentional weight loss (weight 83 kg); no fever, night sweats or change in bowel habits; pale. **CBC and iron studies (R5): hemoglobin 9.2 g/dL, MCV 72 fL, ferritin 15 ng/mL, iron 30 µg/dL, TIBC 450 µg/dL. Hematology/oncology referral** (A9); influenza vaccine |
| 2024-11-11 | outside | hematology/oncology consult: same history; pale, conjunctival pallor; **stool guaiac positive**; labs above. **Iron deficiency anemia**; **ferrous sulfate 325 mg daily started** (A10); referred to gastroenterology for colonoscopy. Letter received 2024-11-18 |
| 2024-12-02 | outside | colonoscopy (A11): **a 30 mm sessile polyp in the ascending colon removed by endoscopic mucosal resection; no other lesions.** Pathology received 2024-12-09: **tubulovillous adenoma with high-grade dysplasia, no invasive carcinoma, margins clear. Colorectal neoplasia** recorded (R6). Surveillance colonoscopy advised in 6 months |
| 2025-01-27 | practice | follow-up: energy better on iron; **hemoglobin 11.8, MCV 78** (A10) |
| 2025-02 (early) | — | cough productive of yellowish sputum begins |
| 2025-03-24 | practice | telehealth: cough with yellow sputum for about 6 weeks, no fever or hemoptysis; chest x-ray ordered, he does not go (P; the order is cancelled in May) |
| 2025-05-05 | outside | hematology/oncology follow-up: 3 months of cough with yellowish sputum and weight loss; no fever, night sweats, hemoptysis. **BP 145/90, HR 88, RR 18.** Decreased breath sounds and dullness to percussion at the right base. **Hemoglobin 13.5, WBC 7.5, platelets 250, sodium 140, potassium 4.2, creatinine 1.0, fasting glucose 100. Chest x-ray: right lower lobe mass.** Referred to pulmonology for bronchoscopy and biopsy (R7). Ferrous sulfate continued. Letter received 2025-05-12 |
| 2025-05-19 | outside | bronchoscopy with endobronchial biopsy and EBUS (A12): right lower lobe endobronchial tumor; prior tracheal site clear. Pathology received 2025-05-23: **non-small cell lung cancer, squamous cell type** (p40 positive, TTF-1 negative); EBUS stations 4R and 4L (contralateral) positive, station 7 negative |
| 2025-05-27 | practice | visit after the diagnosis: wife present; medication review; advance care planning: wife is health care proxy, he wants full treatment (P) |
| 2025-05 (late) | outside | staging (A12): PET-CT: 4.6 cm right lower lobe mass, FDG-avid right hilar and bilateral mediastinal nodes, no distant disease; brain MRI with contrast: no metastasis; no targetable EGFR, ALK or ROS1 alteration; ECOG 1; not a surgical candidate. **Stage IIIB (T2b N3 M0) NSCLC.** Thoracic oncology letter received 2025-06-06 |
| 2025-06-09 to 2025-07-18 | outside | **concurrent chemoradiation** (A13): weekly carboplatin (AUC 2) and paclitaxel (45 mg/m2) with thoracic radiation, 60 Gy in 30 fractions (IMRT), no treatment breaks |
| 2025-08-11 | practice | follow-up after chemoradiation: tired, eating less; weight 79 kg; **hemoglobin 12.6** (A13); **ferrous sulfate stopped** (iron repleted, ferritin 88; A10) |
| 2025-08-25 | outside | **durvalumab consolidation started**, 1500 mg every 4 weeks (A13); practice learns at the next contact |
| 2025-10-06 | practice | follow-up: on durvalumab; influenza and COVID vaccines with oncology's agreement |
| 2025-11-12 to 2025-11-15 | outside | **ED, then admission.** Intermittent dull abdominal pain for 3 months, poorly localized, worse after meals, sometimes to the back; no nausea, vomiting, diarrhea or hematochezia. Exam: **nontender pulsatile mass in the central abdomen, normal bowel sounds. CT angiography: saccular aneurysm of the proximal superior mesenteric artery just past its origin, 3.8 cm, projecting posteriorly.** Also about a week of facial and neck swelling with distended neck veins (R8); **CT chest with contrast: narrowing of the superior vena cava by an enlarging right paratracheal nodal mass. Superior vena cava syndrome secondary to lung cancer**; **SVC stent placed** by interventional radiology through the right femoral vein 2025-11-13 (A14). Vascular surgery consult: elective endovascular repair of the aneurysm after recovery (A15). Durvalumab stopped for progression. Wife calls the practice 2025-11-14; discharge summary received 2025-12-01 |
| 2025-11-24 | practice | post-discharge visit: facial swelling gone; mild after-meal abdominal pain persists |
| 2025-12-08 | outside | **docetaxel 75 mg/m2 every 3 weeks started** by oncology (A13); practice learns at the January visit |
| 2026-01-21 | outside | **endovascular covered stent repair of the SMA aneurysm** (A15); operative note never received |
| 2026-02-23 | practice | follow-up: abdominal pain gone since the repair; on docetaxel; tired for a few days after each cycle |
| 2026-03-09 | outside | vascular surgery follow-up letter: CT angiography, **stent graft patent, aneurysm excluded** (A15) |
| 2026-04 (mid) | — | persistent headache for 2 weeks, not helped by over-the-counter analgesics; forgetfulness and word-finding trouble (wife's account) |
| 2026-05-02 to 2026-05-05 | outside | **ED, then admission: new-onset seizure.** Wife's account as above. Postictal, no focal neurologic deficits; vital signs stable. **MRI brain: multiple well-circumscribed lesions at the gray-white junction with surrounding vasogenic edema. Metastatic carcinoma to brain.** **Levetiracetam 500 mg twice daily and dexamethasone 4 mg twice daily (tapering) started** (A16). Discharge summary received 2026-05-07 |
| 2026-05-11 | practice | post-discharge visit: no further seizures; headache easing; not driving |
| 2026-05-14 | outside | **stereotactic radiosurgery** to the brain lesions (A16); docetaxel continued after |
| 2026-06-08 | practice | follow-up: **dexamethasone taper finished**, no seizures; on levetiracetam and docetaxel |
| 2026-08-17 | outside | follow-up brain MRI ordered by radiation oncology; the report has not reached the practice by today (open item) |
| 2026-09-14 | practice | booked follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Pack-years are given as 80, 30, 40 and "one pack daily for 35 years" (which
  also reads as still smoking in 2025). Resolution: 80 pack-years, quit in early October
  2022, never resumed. The 30, 35 and 40 figures are not carried.
- **R2.** The source's surgical history lists only the hernia repair, but cardiology
  records a femoral artery stent "2 years ago". Resolution: left femoral artery stent in
  May 2021 for claudication, known to the practice only from the patient.
- **R3.** The profile lists lisinopril from the start, but cardiology says a second
  agent was "recently" added for 6 months of systolic pressures of 180 to 190.
  Resolution: HCTZ alone until 2023-02-27, when the PCP adds lisinopril 20 mg. Both
  continue to today. Outside lists that drop lisinopril (2024 pulmonology and
  hematology letters) or HCTZ (2023 practice list as the source has it) are not
  carried on the practice list; outside letters may show them.
- **R4.** Creatinine 1.7 in May 2023 and 1.0 in May 2025 with no change in therapy.
  Resolution: the rise follows starting lisinopril with a right renal artery stenosis;
  it settles to 1.4 in June 2023, 1.2 in August 2023, 1.1 in November 2023 and runs
  1.0 to 1.1 after. eGFR wobbles accordingly.
- **R5.** The 2024 hematology labs were drawn somewhere. Resolution: the PCP draws
  them at the 2024-10-21 visit and the hematology letter cites them.
- **R6.** The source dates colorectal neoplasia to the hematology visit, on a positive
  guaiac. Resolution: iron deficiency anemia is recorded at the hematology visit;
  colorectal neoplasia is recorded on the colonoscopy pathology (December 2024).
- **R7.** The 2025-05-05 note gives the lung-cancer pathology and also refers the
  patient for bronchoscopy and biopsy. Resolution: the chest x-ray is done at that visit,
  the bronchoscopy follows on 2025-05-19, and the pathology arrives after it.
  The case-control study paragraph is dropped.
- **R8.** The source dates superior vena cava syndrome to the November 2025 ED visit
  (and the May 2026 note calls it "prior") but the ED note describes only the
  aneurysm. Resolution: the same stay finds the SVC syndrome (facial and neck swelling,
  distended neck veins, CT chest) and treats it with a stent. The source's unfinished
  sentence about which venous drainage the aneurysm compresses is dropped; no venous
  compression by the aneurysm is recorded.
- **R9.** The emphysema is listed both as "Severe upper lobe predominant emphysema" and
  "(COPD)". Resolution: one condition, recorded at the 2022 pulmonology consult.
- **Left as the source has it:** no low-dose CT lung cancer screening is ever ordered
  despite his age and 80 pack-years; colon cancer screening is declined in 2020 and
  never revisited before the anemia; no echocardiogram after the loud P2 and edema;
  lisinopril continues through the creatinine rise; atorvastatin stays 40 mg with an
  LDL of 120 in a patient with peripheral arterial disease; the 6-month surveillance
  colonoscopy is not done by today. No note remarks on any of these.

## Additions (A): physician to confirm

- **A1.** 2020 new-patient visit (the first practice encounter in the source is a
  "routine follow-up"; seed: about 6 years with the practice).
- **A2.** The 2021 visit that refers him to vascular surgery for the claudication that
  led to the stent.
- **A3.** The 2022 PCP visit that orders the PFTs and the pulmonology consult.
- **A4.** Tiotropium and albuterol from pulmonology: severe emphysema with no
  treatment named; he had been borrowing albuterol.
- **A5.** The referral to cardiology for worsening hypertension.
- **A6.** A renal artery duplex supporting the source's renal artery stenosis (right
  60% or greater, left without significant stenosis) and the creatinine course in R4.
- **A7.** A chest x-ray at the PCP visit for hoarseness and cough (no mass).
- **A8.** Next steps of the tracheal neoplasm: CT chest (1.8 cm polypoid mid-tracheal
  mass, about 50% narrowing), rigid bronchoscopic excision 2024-06-03, pathology squamous
  papilloma without dysplasia; stridor and hoarseness resolve.
- **A9.** The PCP visit that sends him to hematology/oncology.
- **A10.** Ferrous sulfate 325 mg daily for the iron deficiency, hemoglobin 11.8 (MCV 78) in
  January 2025 and 13.5 in May (S; MCV 84 then), ferritin 88 in August 2025, stopped then.
- **A11.** Colonoscopy for the iron deficiency and positive guaiac: 30 mm ascending
  colon tubulovillous adenoma with high-grade dysplasia, resected, no carcinoma.
- **A12.** Diagnostic bronchoscopy with EBUS and staging (PET-CT, brain MRI, biomarkers)
  that give the source's stage IIIB: squamous NSCLC, 4.6 cm right lower lobe mass,
  contralateral mediastinal node, T2b N3 M0.
- **A13.** Standard treatment of unresectable stage IIIB NSCLC: concurrent weekly
  carboplatin/paclitaxel with 60 Gy thoracic radiation (June to July 2025), durvalumab
  every 4 weeks from 2025-08-25, stopped at progression in November 2025, docetaxel
  every 3 weeks from 2025-12-08 to today. Hemoglobin 12.6 after chemoradiation.
- **A14.** SVC stent for the SVC syndrome (2025-11-13), with resolution of the swelling.
- **A15.** Repair of the symptomatic 3.8 cm SMA aneurysm: endovascular covered stent
  2026-01-21; follow-up CT angiography patent and excluded; the pain resolves.
- **A16.** Treatment of the brain metastases and seizure: levetiracetam 500 mg twice
  daily (continuing), dexamethasone 4 mg twice daily tapered off by early June 2026,
  stereotactic radiosurgery 2026-05-14. No further seizures.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine care there (Tdap 2016) with normal
  results.
- Visits for hypertension, PAD and COPD about every 4 to 6 months, then closer during
  cancer care; vitals; BMP and lipids about yearly; CBC when the anemia is followed. BP
  about 126 to 132 / 78 to 82 on HCTZ until late 2022; 145 to 150 / 88 to 92 after
  lisinopril in spring 2023; 130s/80s from late 2023 on. Weight 92 kg to 2023, 87.5 kg
  late 2023, 83 kg late 2024, 79 kg after chemoradiation, about 77 kg in 2026. SpO2 93
  to 95% on room air once the emphysema is known.
- Annual wellness visits (Medicare); PHQ-2 negative; AUDIT-C low.
- Influenza vaccine each fall; COVID vaccines (primary series spring 2021 and boosters
  at a pharmacy); PCV20 in 2023; Shingrix series in 2022. Hepatitis C and HIV screening
  once (2020, nonreactive).
- Refills, pharmacy and prior-authorization traffic (tiotropium), portal messages about
  results, refills and scheduling, a missed lab, records requests, oncology and radiation
  oncology appointment letters, driving restriction counseling after the seizure.
- Booked visit after 2026-08-27: routine follow-up 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
