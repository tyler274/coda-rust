# Build requirements: unit-2082 (Derek Plante, MRN00002082)

Source: synthetic_hospital patient 2082, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (odds ratios, case-control study
tables, positive predictive value lectures, study enrollment).

## Who he is (S)

Derek Plante, born 1976-10-26, male, White, not Hispanic or Latino, English, lives in
Ludlow (01056), commercial insurance. Construction worker. Current cigarette smoker,
30 pack-years (the figure every source note gives; R5). Occasional social alcohol. No
known allergies. No surgical history. Family history: father with COPD; a paternal
uncle with lung cancer (R2). A sister, who brings him to the ED in 2026 (S). Marital
status single (no spouse appears anywhere in the source).

Joined the practice 2020-08-19 (A1), a new-patient visit, bringing: chronic
obstructive pulmonary disease and tobacco use disorder (current smoker), on
**tiotropium 18 mcg inhaled once daily** (dry-powder capsule) and **albuterol HFA
90 mcg, 1 to 2 puffs every 4 to 6 hours as needed for shortness of breath**. Both
continue unchanged through the whole chart. Height 178 cm, weight 84 kg at joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-08-19 | practice | new-patient visit (A1): history as above; exam normal; screening labs normal; smoking cessation advised, not ready |
| 2020-09-02 | outside | prior primary-care records received, including spirometry from 2018 (A2): post-bronchodilator FEV1/FVC 0.62, FEV1 71% predicted, consistent with COPD |
| 2022-03-28 | practice | annual physical, age 45. No complaints. Worried about his own lung-cancer risk compared with non-smokers because a 52-year-old friend with a similar smoking history was just diagnosed with lung cancer. BP 130/85, HR 78, RR 16. Exam unremarkable. Family history as R2. Risk discussed in plain terms (no quiz, no figures); **low-dose CT chest ordered** (R3) |
| 2022-09-12 | outside | low-dose CT chest (screening): **6 mm solid non-calcified nodule, right lower lobe**, Lung-RADS 3; otherwise normal. Practice refers to pulmonology the next day |
| 2022-09-28 | outside | pulmonology consult letter: no complaints, anxious about the nodule and asks how likely it is cancer; exam unremarkable; family history father COPD, paternal uncle lung cancer. Discussed that the chance depends on his age, smoking history and the nodule's features and that most nodules this size are benign, but his is concerning given his history. **Pulmonary nodule on the problem list** (the source's "solitary pulmonary nodule with high suspicion for malignancy", R4). Plan: repeat low-dose CT in 6 months |
| 2024-09-16 | practice | sick visit (P): dry cough for about 3 months and unintentional loss of 15 lb (84 to about 77 kg); no fever, night sweats or hemoptysis; breath sounds decreased over the right upper lobe. Chest X-ray ordered, urgent pulmonology referral |
| 2024-09-17 | outside | chest X-ray, 2 views: **right upper lobe mass**. Lung mass, suspected malignancy, on the problem list (R6) |
| 2024-09-27 | outside | pulmonology visit: persistent dry cough and unintentional weight loss of 15 lb over 3 months; denies fever, night sweats, hemoptysis. Afebrile, BP 130/85, HR 78, RR 16. Decreased breath sounds right upper lobe. Labs: hemoglobin 14 g/dL, WBC 7.0 x10^3/uL, platelets 250 x10^3/uL, sodium 140, potassium 4.2, creatinine 1.0, glucose 95. Chest X-ray reviewed; CT and tissue sampling arranged |
| 2024-10-01 | outside | CT chest with contrast: **4 cm right upper lobe mass with mediastinal lymphadenopathy**; the 6 mm right lower lobe nodule is unchanged (R4) |
| 2024-10-09 | outside | bronchoscopy with endobronchial ultrasound-guided needle sampling (A3); pathology 2024-10-11: **non-small cell lung carcinoma** (not further subtyped on the small sample). **Malignant neoplasm of lung** replaces the lung mass on the problem list |
| 2024-10-16 | outside | staging PET-CT (A4): FDG-avid right upper lobe mass and ipsilateral mediastinal nodes; no distant disease |
| 2024-10-18 | outside | MRI brain with and without contrast (A4): no metastases |
| 2024-10-22 | outside | oncology consult (A5): stage IIIA (T2a N2 M0) NSCLC, unresectable after multidisciplinary review; plan concurrent chemoradiation, then durvalumab |
| 2024-10-24 | practice | follow-up visit (P) on the diagnosis; smoking cessation pushed again, he cuts down but declines medication |
| 2024-11-04 to 2024-12-13 | outside | concurrent chemoradiation (A5): **weekly carboplatin AUC 2 and paclitaxel 45 mg/m2** with **60 Gy in 30 fractions** to the right upper lobe mass and involved mediastinum; completed. Treatment summary received 2024-12-20 |
| 2025-01-08 | outside | post-treatment CT chest (A5): right upper lobe mass decreased to 2.3 cm, mediastinal nodes decreased; no new disease |
| 2025-01-13 | outside | **durvalumab 1500 mg IV every 4 weeks started** (A5), planned for 12 months |
| 2025-01-22 | practice | follow-up (P): tired after chemoradiation, weight 74 kg; COPD stable |
| 2025-04-07 | outside | surveillance CT chest (A5): stable, no new disease; oncology letter 2025-04-14, durvalumab continues |
| 2025-05-09 | practice | **last seen.** Follow-up (P), weight 76.5 kg; COPD stable on tiotropium and albuterol; still smoking, fewer cigarettes; FIT kit given |
| 2025-07-14 | outside | surveillance CT chest (A5): stable; oncology letter 2025-07-21 |
| 2025-09-14 | outside | ED (Sunday): two days of worse shortness of breath with productive cough and green sputum. RR 22, SpO2 88% on room air; tachypneic, diffuse wheezes. Arterial gas pH 7.32, PaCO2 55 mmHg, PaO2 70 mmHg. Chest X-ray: increased bronchial wall thickening, no consolidation. **Admitted for COPD exacerbation** |
| 2025-09-14 to 2025-09-17 | outside | admission (A6): nebulized albuterol/ipratropium, prednisone 40 mg daily for 5 days, azithromycin for 5 days, noninvasive ventilation the first night; improved, discharged on room air home on his inhalers to finish the prednisone and azithromycin. Discharge summary received 2025-09-19 |
| 2025-10-13 | outside | surveillance CT chest (A5): stable; oncology letter 2025-10-20, durvalumab continued |
| 2025-12-29 | outside | last durvalumab dose, 12 months completed (A5); oncology letter 2026-01-12, weight 76 kg, surveillance CT planned for spring |
| 2026-03-28 | outside | ED (Saturday), age 49: brought by his sister for a week of progressive confusion, lethargy and poor appetite; she reports nearly 7 kg of unintentional weight loss over two months and a persistent cough with occasional blood-streaked sputum. BP 116/74, HR 84. Disoriented and drowsy; no peripheral edema. Chest imaging: **centrally located lung mass**. Labs (A7): sodium 117, serum osmolality 247 mOsm/kg, urine osmolality 486 mOsm/kg, urine sodium 58 mmol/L, potassium 3.9, creatinine 0.7, glucose 104, TSH 1.9, AM cortisol 17 mcg/dL. **SIADH**. Admitted |
| 2026-03-28 to 2026-04-02 | outside | admission (A7): CT head without contrast normal; CT chest with contrast: central right upper lobe mass 5.8 cm, larger than in 2025, with enlarged mediastinal nodes (progression). Fluid restriction 1 L/day and sodium chloride 1 g three times daily; sodium 128 at discharge and mentation back to baseline; hemoptysis limited to streaks, hemoglobin 12.6. Oncology saw him: progression after chemoradiation and durvalumab, second-line systemic therapy as outpatient. Discharge summary received 2026-04-06 |
| 2026-04-16 | outside | **docetaxel 75 mg/m2 every 21 days started** (A8); oncology letter 2026-04-20, sodium 131 on fluid restriction and salt tablets |
| 2026-07-08 | outside | restaging CT chest after 4 cycles (A8): mass 5.6 cm, nodes stable, stable disease; oncology letter 2026-07-13, sodium 133, docetaxel continues |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 is a "routine annual physical", so he was already a patient:
  he joined 2020-08-19 (A1).
- **R2.** Family history differs: encounter 0 says no family history of cancer,
  encounter 2 says lung cancer in his father, the profile and other notes say father
  with COPD and paternal uncle with lung cancer. Resolution: the profile holds
  throughout. Encounter 0 records no cancer in a parent or sibling with the uncle's
  lung cancer noted; no note says his father had cancer.
- **R3.** The source calls the CT "screening", ordered at the physical, yet he is 45,
  below the age at which screening is recommended. Resolution: the practice orders a
  low-dose CT after a shared discussion of his heavy smoking, COPD and family history,
  knowing he is under the usual age; insurance approves it. He delays scheduling it
  until September 2022. The source's "recent screening CT" at pulmonology is this
  study, and the pulmonology letter is a first consult (the source's "returns" is not
  carried).
- **R4.** The 2022 nodule is 6 mm in the right lower lobe; the 2024 mass is 4 cm in the
  right upper lobe. Resolution: they are different lesions. The 2024 CT notes the
  right lower lobe nodule unchanged at 6 mm; the cancer arises in the right upper lobe.
  The source's "high suspicion for malignancy" label stays as pulmonology's concern in
  the 2022 letter; the chart condition is Pulmonary nodule.
- **R5.** Every source note gives 30 pack-years even as years pass. Resolution: the
  chart carries "about 30 pack-years" throughout; no other figure.
- **R6.** The source dates "suspected malignant neoplasm of lung" to encounter 0.
  Resolution: suspicion starts with the 2024-09-17 chest X-ray mass (a lung-mass
  condition, R91.8, noted as suspected malignancy), and the 2024-10-11 pathology
  replaces it with malignant neoplasm of lung (non-small cell). Nothing before
  September 2024 mentions a suspected cancer.
- **R7.** The 2025 ED note calls him a patient with a solitary pulmonary nodule and the
  2026 ED note lists only "suspected" cancer. Resolution: both ED records state known
  stage III non-small cell lung cancer (on durvalumab in 2025, treatment completed in
  2026), as an ED would from his records.
- **R8.** The 2026 ED note says he has not seen a physician in many years. Not carried:
  he has been under oncology care throughout.
- **R9.** The 2025 ED arterial gas stays (ED setting). The quiz text and the study
  enrollment of encounters 0 to 2 are dropped.
- **Left as the source has it:** the 6-month repeat low-dose CT that pulmonology asks
  for in September 2022 never happens. No one schedules it and no note comments on it;
  the 2023 and early 2024 physicals list the nodule as followed by pulmonology. No
  smoking-cessation drug is ever started (he declines each time it is offered).

## Additions (A): physician to confirm

- **A1.** The 2020-08-19 new-patient visit (the source starts with an annual physical).
- **A2.** 2018 spirometry in prior records (FEV1/FVC 0.62, FEV1 71% predicted) to
  support the COPD the source lists without evidence.
- **A3.** The bronchoscopy with endobronchial ultrasound-guided sampling that
  produced the source's biopsy (the source gives only "biopsy").
- **A4.** Staging PET-CT and brain MRI, findings as in the timeline, which any
  oncologist obtains before treating a newly diagnosed NSCLC with mediastinal nodes.
- **A5.** The standard treatment of the source's unresectable stage III NSCLC, never
  treated in the source: oncology consult, concurrent weekly carboplatin/paclitaxel
  with 60 Gy radiation, then durvalumab 1500 mg every 4 weeks for 12 months, with
  surveillance CTs as dated (2.3 cm after treatment, then stable through October
  2025).
- **A6.** The 2025 hospital course after the source's "admitted for COPD exacerbation":
  standard treatment as listed.
- **A7.** The laboratory evidence for the source's SIADH (the source gives no sodium),
  the CT head for confusion, the CT chest showing the central mass as progression
  (5.8 cm), and the standard SIADH treatment (fluid restriction, sodium chloride
  tablets) with the sodium course 117 to 128.
- **A8.** Second-line docetaxel for the progression, with the April and July 2026
  oncology letters (sodium 131, then 133; mass 5.6 cm, stable disease).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart size, no effusion, no consolidation where not stated, normal neurologic exam
  when he is well). An abnormal finding appears only as the source or this file gives
  it. A report may carry the descriptors its kind always carries (how a size was
  measured, Lung-RADS category for the 2022 nodule, the biopsy access route), but
  nothing that adds a territory, a severity grade or a symptom, or changes the
  diagnosis or plan. No emphysema, coronary calcification or other incidental finding
  is reported.
- Records from a prior practice may mention routine care there, with normal results.
- Visits: new-patient 2020, physicals 2021-03, 2022-03 (encounter 0), 2023-03 and
  2024-04; COPD follow-ups in 2021 and 2023 (one missed and rebooked); the 2024 sick
  visit, the post-diagnosis visit, January 2025 and the last visit 2025-05-09. Vitals,
  yearly CMP, CBC and lipids, all normal; A1c screening normal; hepatitis C and HIV
  screening once (negative).
- Prevention: yearly FIT from 2022 (age 45), all negative; influenza vaccine each fall
  2020 to 2024; COVID vaccines (primary series spring 2021 at a pharmacy, updated
  doses in the falls of 2022 to 2024); PCV20 in 2021 for COPD and smoking; Tdap 2021.
  PHQ-2 and AUDIT-C screens, negative or low.
- Smoking-cessation counseling at most visits, always declined medication.
- Refills of tiotropium and albuterol, pharmacy and prior-authorization traffic, portal
  messages about results and scheduling, a post-discharge nurse call in September 2025
  and a portal message from him in April 2026. After 2025-05-09 the practice's lists
  are not updated; outside records are filed as received.
- No booked visit after today (seed booked: false).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002082 on 2025-05-09. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
