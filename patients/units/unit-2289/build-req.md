# Build requirements: unit-2289 (William Casey, MRN00002289)

Source: synthetic_hospital patient 2289, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("greatest risk for death from").

## Who he is (S)

William Casey, born 1957-11-16, male, White, not Hispanic or Latino, English,
Medicare, lives in Holyoke (01040), married (his wife brings him to the ED twice).
Retired naval shipyard worker: 30 years in the yards, retired at 55 (about 2013), with
occupational asbestos exposure. Former smoker, 90 pack-years, quit in 2021 after
several earlier attempts (R1). Alcohol socially, a beer once or twice a month. Exercises
occasionally. Allergies: none known (NKDA) throughout.

Family history: father lung cancer, died at 68, a heavy smoker; mother emphysema; a
sibling (brother) with coronary artery disease.

Surgical/procedure history as the source lists it, each dated in the timeline: CT-guided
transthoracic needle biopsy of the lung mass (2023-11), right thoracentesis with chemical
pleurodesis for recurrent effusion (2024-09), central venous port placement (2025-02).
No colonoscopy or other cancer screening done, ever (left as the source has it).

**Joined the practice 2023-06-13** (A1, new-patient visit). Brought: hyperlipidemia on
**atorvastatin 40 mg daily** and **aspirin 81 mg daily** (both from his prior practice),
asbestos exposure, former smoking, a history of a recent pneumonia (spring 2023, treated
as an outpatient with oral antibiotics by his prior office, R2), and years of morning
productive cough with frequent chest infections. Height 178 cm, weight 82 kg.
Not known at joining: bronchiectasis, any lung cancer, effusions, SIADH.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-06-13 (Tue) | practice | new-patient visit (A1). History as above. Five years of frequent chest infections, each treated with antibiotics; a morning cough with large amounts of yellow-green sputum. Exam: digital clubbing, scattered rhonchi at both bases; otherwise normal. **Referred to pulmonology.** Labs: CMP, CBC, lipid panel, hepatitis C antibody, HIV screen (normal). PCV20 and Tdap given. |
| 2023-07-17 (Mon) | outside | pulmonology consult letter: copious foul yellow-green sputum every morning, worst on waking; frequent infections for 5 years, each needing antibiotics; denies weight loss, fever, night sweats. Exam: digital clubbing, scattered rhonchi over both bases. Chest CT (2023-07-11): dilated, thick-walled bronchi with mucus plugging and cystic changes in both lower lobes. **Bronchiectasis diagnosed.** Plan: airway clearance (A8), follow up in the fall. Letter received 2023-07-24. |
| 2023-09-08 (Fri) | practice | routine health maintenance (Medicare wellness visit); age 65; no major complaints; on atorvastatin and aspirin; vitals normal; exam unremarkable. The quiz plan becomes counseling (R3): his largest health risk is lung cancer (90 pack-years, asbestos, father's lung cancer); pulmonology already has a chest CT planned for October, so no separate screening order. Shingrix dose 1. |
| 2023-10-03 (Tue) | practice | nurse visit: influenza (high-dose) and updated COVID vaccines; RSV vaccine. |
| 2023-10-10 (Tue) | outside | pulmonology follow-up letter: chronic cough and progressive dyspnea; denies chest pain, hemoptysis, weight loss. Exam: bilateral inspiratory crackles at the bases. Chest x-ray: interstitial fibrosis and pleural plaques. High-resolution CT: diffuse subpleural reticulation and honeycombing, **and right hilar soft-tissue fullness, not characterized without IV contrast; contrast CT recommended (R4)**. PFTs: restrictive pattern (no numbers). Impression: **asbestosis** with pleural plaques (R5); **suspected malignant neoplasm of lung** (R4). Pulmonologist discussed the long-term course (progressive fibrosis, raised risk of lung cancer and mesothelioma) and ordered a contrast chest CT for late November. Letter received 2023-10-17. |
| 2023-10-19 (Thu) | practice | telephone: PCP reviews the letter with the patient; suspected lung malignancy added to the problem list; contrast CT pending with pulmonology. |
| 2023-11-18 (Sat) | outside | ED, brought by his wife: progressive confusion, fatigue and constipation over several days. Lethargic, mildly dehydrated; slowed reflexes, difficulty concentrating. **Calcium 13.9 mg/dL, phosphate 1.9 mg/dL, PTH low (9 pg/mL, R6).** Chest x-ray: **4.5-cm right hilar mass**. **Hypercalcemia.** Admitted 2023-11-18 to 2023-11-24 (A2): IV saline, **zoledronic acid 4 mg IV once**; contrast CT chest: the right hilar mass and a small right pleural effusion; CT abdomen/pelvis and MRI brain without metastases; **CT-guided transthoracic needle biopsy (2023-11-21): small cell carcinoma**; **diagnostic right thoracentesis (2023-11-22): cytology positive for malignant cells → malignant pleural effusion (R7)**. Stage IV, extensive-stage small cell lung cancer. Discharge calcium 10.2. Received as a short ED note and a brief discharge summary, 2023-11-27. |
| 2023-11-28 (Tue) | practice | post-discharge visit with his wife: confusion resolved; BMP calcium 9.6 (A2). **Malignant neoplasm of lung (small cell, extensive stage) and malignant pleural effusion added; suspected-malignancy entry resolved; hypercalcemia resolved.** Oncology appointment set by the hospital. |
| 2023-12-11 | outside | **first-line carboplatin + etoposide + atezolizumab** starts at an outside oncology group (A3); 4 cycles every 3 weeks through 2024-03; then **atezolizumab maintenance every 4 weeks**. The practice learns this from the patient (sparse) and a brief oncology summary letter received 2024-04-08. |
| 2024-04-16 (Tue) | practice | follow-up: 4 cycles done, on atorvastatin, aspirin and maintenance immunotherapy; Shingrix dose 2; weight 78 kg. |
| 2024-09-06 (Fri) | outside | right thoracentesis at the cancer center for a recurrent right pleural effusion, with initial improvement (S, "despite initial improvement"); known only from the patient's account. |
| 2024-09-14 (Sat) | outside | ED: dyspnea, cough and chest pain worse lying down, back within a week of the drainage; no trauma. **BP 100/60, HR 110, RR 22, SpO2 92% RA.** JVP elevated, heart sounds muffled. Echo: **large pericardial effusion with tamponade**. **Pericardiocentesis; cytology malignant cells.** Admitted 2024-09-14 to 2024-09-20; during the stay the reaccumulated right effusion is drained with **chemical (talc) pleurodesis (2024-09-18)** (R8). Received as a short ED note 2024-09-23; the pleurodesis is known from the patient. Atezolizumab continued by oncology after drainage (A3). |
| 2024-09-26 (Thu) | practice | post-hospital visit: malignant pericardial effusion with tamponade (drained) recorded; recurrent malignant pleural effusion, pleurodesis done; breathing better. |
| 2024-10-08 (Tue) | practice | nurse visit: influenza and COVID vaccines. |
| 2024-12-10 (Tue) | practice | Medicare wellness visit; weight 77 kg; PHQ-2 0. |
| 2025-02 | outside | restaging shows progression (A4); atezolizumab stopped; **implanted venous access port placed 2025-02-25**. Known from the patient (portal message). |
| 2025-03-08 (Sat) | outside | admitted for scheduled chemotherapy (A4: cycle 1 of carboplatin + etoposide rechallenge) through the recently placed port. Day 3 (2025-03-10): fever 38.9 °C, chills, shortness of breath; no cough, sputum or urinary symptoms. **BP 110/70, HR 110, RR 22, SpO2 94% RA.** Mild tenderness over the catheter site without erythema or discharge; lungs clear. **WBC 12.0, Hgb 10.2, platelets 250**; urinalysis normal; blood cultures pending at the time, later growing **coagulase-negative staphylococci in port and peripheral sets (A4)**. Chest x-ray: no infiltrate or consolidation. **Catheter-related bloodstream infection.** IV vancomycin 14 days with antibiotic lock, port kept; repeat cultures negative (A4). Discharged 2025-03-18. Brief discharge summary received 2025-03-20. |
| 2025-03-25 (Tue) | practice | post-discharge visit: no fever; CBC: WBC normal, Hgb 10.6 (A4); weight 76 kg. Cycles 2 to 4 outpatient through 2025-06 (A4). |
| 2025-06-17 (Tue) | practice | telehealth follow-up: finishing rechallenge; no changes. |
| 2025-09 | outside | progression; **lurbinectedin every 3 weeks** starts 2025-09-05 (A5); most recent dose 2025-10-17. Practice learns at the next contact. |
| 2025-10-07 (Tue) | practice | nurse visit: influenza and COVID vaccines. |
| 2025-11-16 (Sun) | outside | ED, brought by his wife (his 68th birthday): increasing confusion and lethargy for 3 days, like his earlier confusion episode; drinking normal amounts but little appetite; chemotherapy one month earlier. Vitals normal. Oriented to person only; euvolemic, normal skin turgor, no edema. Foley catheter placed. Labs (A6): **sodium 119, serum osmolality 252, urine osmolality 470, urine sodium 54**; TSH 2.1 and AM cortisol 15.8 (normal). **SIADH.** Admitted 2025-11-16 to 2025-11-20: fluid restriction, cautious hypertonic saline, then **sodium chloride 1 g tablets three times daily** and a 1.2 L/day fluid restriction; discharge sodium 128. ED note received 2025-11-21. |
| 2025-11-25 (Tue) | practice | post-discharge visit: oriented, back to baseline; SIADH added; sodium chloride tablets recorded; BMP sodium 131. On lurbinectedin. |
| 2025-12-09 (Tue) | practice | lab only: BMP sodium 134. |
| 2026-03-03 (Tue) | practice | **last seen.** Follow-up and Medicare wellness visit: sodium 137; **sodium chloride tablets stopped**, fluid restriction relaxed; SIADH carried as paraneoplastic history (resolved). Weight 75.5 kg. On lurbinectedin, atorvastatin, aspirin. |
| 2026-07-04 (Sat) | outside | ED: progressive shortness of breath and productive cough with occasional streaks of blood; no chest pain or fever; appetite down; **5 kg unintended weight loss over 3 months**; multiple past bronchitis episodes, never hospitalized for a respiratory illness. **T 36.7 °C, BP 132/84, HR 94, SpO2 94% RA.** Decreased breath sounds and dullness over the right middle and lower fields; no crackles or wheezes; trachea deviated to the right. Chest x-ray: dense opacity in the right mid and lower zones with mediastinal shift to the right. CT chest: **central right bronchial mass with volume loss of the right lung → obstructive atelectasis.** Referred back to oncology. Short ED note received 2026-07-09; filed, lists not updated. |
| 2026-07 to 08 | outside | oncology stops lurbinectedin and gives **palliative radiation to the right central airway mass** (A7); known from his wife's portal message 2026-08-11. |
| 2026-09-10 (Thu) | practice | booked follow-up visit. |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Smoking is given as 35 years quit a decade ago, 40 pack-years quit 5 years
  ago, and 90 pack-years quit 2 years ago. Resolution: 90 pack-years (the profile),
  quit in 2021, two years before his first encounter. The 35-year and 40-pack-year
  figures are not carried. Later outside notes that still say "quit two years ago" are
  copying an old line, as outside notes do.
- **R2.** "History of recent pneumonia" sits beside "never hospitalized for respiratory
  illness" and frequent antibiotic-treated infections. Resolution: one outpatient
  pneumonia in spring 2023 treated by his prior office, reported at joining (history
  only, resolved).
- **R3.** Encounter 1's plan is a quiz stem. Resolution: the PCP counsels him that lung
  cancer is his biggest risk and relies on the planned pulmonology CT; no low-dose
  screening CT is ordered, consistent with "no other cancer screening procedures".
- **R4.** The source stamps "suspected malignant neoplasm of lung" at the September visit
  with no evidence, and a 4.5-cm hilar mass appears five weeks after an HRCT that
  mentions none. Resolution: the October HRCT reports right hilar soft-tissue fullness
  needing a contrast study, and pulmonology records the suspected malignancy then
  (practice list 2023-10-19). The July CT shows bronchiectasis only.
- **R5.** "Asbestos exposure" plus pleural plaques, basal reticulation and honeycombing
  with a restrictive pattern in a 30-year shipyard worker: pulmonology names it
  asbestosis. Both asbestos exposure and asbestosis are on the list.
- **R6.** PTH is given only as "low": carried as 9 pg/mL.
- **R7.** The source makes him Stage IV extensive-stage from the start, stamps the
  malignant pleural effusion in September 2024, yet calls that encounter a recurrent
  effusion. Resolution: cancer is diagnosed in November 2023; the malignant pleural
  effusion found at staging is what makes it stage IV; it recurs in September 2024.
  "Small cell" (encounter 6) is the biopsy result.
- **R8.** The thoracentesis with chemical pleurodesis on the surgical list is dated to
  the September 2024 admission, after the second recurrence.
- **R9.** The source's ages (66 in March 2025, 67 in November 2025) follow the seed's
  birth date: 67 and 68.
- **R10.** Encounter 1 says "no family history of cancer"; the father's lung cancer is
  kept and that statement is dropped.
- **R11.** The source puts SIADH ("paraneoplastic syndrome history") on every list from
  the start. Resolution: SIADH first happens in November 2025 and is history after it
  resolves in March 2026. Malignant pleural effusion, catheter infection and atelectasis
  likewise start at their own encounters.
- **Left as the source has it:** no colonoscopy, FIT or other cancer screening is ever
  done or discussed; aspirin 81 mg continues through chemotherapy and later hemoptysis
  without comment.

## Additions (A): physician to confirm

- **A1.** The 2023-06-13 new-patient visit and pulmonology referral (encounter 1 reads as
  an established patient "returning"); its baseline labs and vaccines are P.
- **A2.** The November 2023 admission course: IV saline, zoledronic acid 4 mg once,
  staging contrast CT chest (right hilar mass, small right effusion), CT abdomen/pelvis
  and MRI brain without metastases, biopsy result small cell carcinoma, effusion
  cytology positive, discharge calcium 10.2; practice calcium 9.6 on 2023-11-28.
- **A3.** Standard first-line therapy for extensive-stage small cell lung cancer:
  carboplatin + etoposide + atezolizumab ×4 (2023-12-11 to 2024-03), then atezolizumab
  every 4 weeks until progression in 2025-02, continued after the September 2024
  drainage.
- **A4.** Progression in 2025-02, port placed 2025-02-25 (the source's "recently placed"
  catheter), carboplatin + etoposide rechallenge (cycle 1 inpatient from 2025-03-08,
  cycles 2 to 4 outpatient through 2025-06). Blood cultures grow coagulase-negative
  staphylococci (port and peripheral); vancomycin IV 14 days with antibiotic lock,
  port retained, repeat cultures negative. Practice CBC 2025-03-25: WBC normal, Hgb 10.6.
- **A5.** Lurbinectedin every 3 weeks from 2025-09-05 (the source's "chemotherapy one
  month ago" in November 2025), last dose before the ED 2025-10-17.
- **A6.** The labs that make the source's SIADH a diagnosis: sodium 119, serum osmolality
  252, urine osmolality 470, urine sodium 54, TSH 2.1 and AM cortisol 15.8 (normal); treatment as in
  the timeline; sodium 128 at discharge, 131 (2025-11-25), 134 (2025-12-09), 137
  (2026-03-03, tablets stopped).
- **A7.** After the July 2026 ED: oncology stops lurbinectedin and gives palliative
  radiation to the central right bronchial mass; nothing further is known by today.
- **A8.** Bronchiectasis airway clearance: an oscillating positive expiratory pressure
  device twice daily (no drug).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Records: **sparse**. Outside care reaches the practice as short letters, short ED notes,
  brief discharge summaries or the patient's and his wife's account, with few numbers,
  as each timeline row says. No separate imaging or pathology reports reach the chart.
- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Weights: 82 kg (2023), 78 (2024-04), 77 (2024-12), 76 (2025-03), 75.5 (2026-03); the
  July 2026 ED weight is about 70.5. Blood pressure and other vitals normal at practice
  visits.
- Vaccines: PCV20 and Tdap 2023-06; Shingrix 2023-09 and 2024-04; RSV 2023-10; influenza
  (high-dose) and COVID each fall 2023, 2024, 2025. Hepatitis C and HIV screening once
  (2023, negative). PHQ-2 at wellness visits (0 to 1); AUDIT-C low.
- Refills of atorvastatin and aspirin, pharmacy traffic, portal messages from him and
  his wife about scheduling, results and records, a cancelled visit, records requests.
- Booked visit after 2026-08-27: 2026-09-10 follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002289 on 2026-03-03. Practice contacts after that day are
only messages, refills and records filed as received; the practice's problem and
medication lists stay as they were on 2026-03-03. A visit is booked after today.
