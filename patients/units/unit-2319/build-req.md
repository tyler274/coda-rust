# Build requirements: unit-2319 (Andrew Marino, MRN00002319)

Source: synthetic_hospital patient 2319, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, its "is shown" images or its quoted dialogue word for word.

## Who he is (S)

Andrew Marino, born 1961-03-08, male, White, not Hispanic or Latino, English, private
PPO (employer retiree health plan). Retired construction supervisor. Married; his wife
comes to oncology visits (S, encounter 2). Former smoker, 40 pack-years, quit in
August 2022 (R1). Alcohol: social, 1 to 2 drinks a week. Allergies: iodinated contrast
(hives/urticaria) and sulfonamide antibiotics (rash), both remote and known when he
joins. Family history: father lung cancer, died at 67; paternal uncle renal cell
carcinoma at 72; mother hypertension, alive at 85.

Surgical history (S, years re-dated by R2): left upper lobectomy (January 2023), left
radical orchiectomy (January 2024), left radical nephrectomy (January 2025).

Joined the practice 2022-05-10 (A1), age 61, still smoking. Brought: essential
hypertension on lisinopril 5 mg daily. No other problems or drugs. Height 178 cm,
weight 88 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-05-10 | practice | **new-patient visit** (A1). Hypertension on **lisinopril 5 mg daily** (S home med); BP 136/84. Smoker, 40 pack-years, advised to quit; LDCT screening ordered (A2). Prevention per P |
| 2022-08 | practice (portal) | reports he has **quit smoking** (R1) |
| 2022-09-06 | outside imaging | **low-dose CT** (A2): 1.9 cm solid left upper lobe nodule, Lung-RADS 4B; no other nodules; no adenopathy |
| 2022-09-27 | outside | PET-CT (A2): left upper lobe nodule FDG-avid, SUVmax 6.2; no nodal or distant uptake |
| 2022-10-18 | outside pulmonology | navigational bronchoscopic biopsy (A2): adenocarcinoma, non-small cell lung cancer. **Malignant neoplasm of lung diagnosed.** Referred to thoracic surgery |
| 2022-12-13 | practice | pre-operative visit (P) |
| 2023-01-10 to 2023-01-13 | outside hospital | **left upper lobectomy** (S, R2) with node dissection. Pathology (A2): adenocarcinoma 2.0 cm, 0 of 9 nodes, margins negative, stage IA (pT1b N0). No adjuvant therapy. Discharged on **acetaminophen 650 mg every 6 hours as needed for pain** (R3) |
| 2023-01-24 | practice | post-discharge visit. Lung cancer becomes **history of stage I NSCLC, status post left upper lobectomy** |
| 2023-07 | outside thoracic surgery | 6-month surveillance CT chest: post-lobectomy changes, no recurrence (A2) |
| 2023-12-12 | practice | about 3 weeks of a firm painless left testicular lump (A3); scrotal ultrasound and labs ordered, urgent urology referral |
| 2023-12-14 | outside imaging | scrotal ultrasound (A3): 2.8 cm heterogeneous solid intratesticular mass, left testis; right testis normal |
| 2024-01-09 | outside urology | **left radical orchiectomy** (S, R2). Pathology (A3): non-seminomatous germ cell tumor, 3.0 cm, confined to the testis, cord margin negative. **Left testicular NSGCT diagnosed.** AFP before surgery 412 ng/mL |
| 2024-02 | outside oncology | staging CT chest/abdomen/pelvis without contrast: no adenopathy or metastases (A3). AFP 3 weeks after surgery 188 ng/mL, not normalizing; stage IS. **BEP (bleomycin, etoposide, cisplatin), 3 cycles**, started 2024-02-12 (S "combination chemotherapy", R4) |
| 2024-04-15 | outside oncology | BEP completed (3rd cycle ended); AFP 4 ng/mL (normal) (A3) |
| 2024-06-11 | practice | creatinine 1.8 (was 1.15) after cisplatin: **CKD stage 3b diagnosed** (R5). BP 148/90: **furosemide 20 mg daily started** (R6). Testicular cancer becomes history |
| 2024-07-16 | practice | cough and exertional breathlessness since about June (S symptoms, P visit); SpO2 93% room air; fine basal crackles; chest x-ray: bibasilar reticular opacities, post-lobectomy changes; referred to pulmonology |
| 2024-08-13 | outside pulmonology (S encounter 0) | 6 months after starting BEP: persistent nonproductive cough, progressive dyspnea on exertion. SpO2 91% on room air. Fine dry crackles at both bases. HRCT (R7, without contrast): bibasilar subpleural reticulation with traction bronchiectasis and ground-glass, post-lobectomy changes on the left. PFTs: reduced TLC 64% predicted and DLCO 41% predicted (A4). 6-minute walk: nadir SpO2 86% on room air (A4). **Bleomycin-induced pulmonary fibrosis diagnosed.** **Prednisone 10 mg daily started** and **oxygen 2 L/min by nasal cannula, continuous** (R8). Bleomycin will never be given again |
| 2024-08-26 | practice | follow-up: **iatrogenic immunosuppression** (prednisone) and **dependence on supplemental oxygen** on the list (R8). SpO2 95% on 2 L |
| 2024-11-07 | outside imaging (oncology surveillance) | CT chest without contrast: four new right-lung nodules, 4 to 9 mm, fibrosis unchanged. MRI abdomen and pelvis with and without gadolinium (contrast allergy, R7): 6.3 cm enhancing solid mass, lower pole left kidney, renal vein patent, no adenopathy; consistent with renal cell carcinoma (A5) |
| 2024-11-25 | outside urology | consult: radical nephrectomy recommended (A5) |
| 2024-12-17 | practice | pre-operative visit (P) |
| 2025-01-08 to 2025-01-11 | outside hospital | **left radical nephrectomy** (S, R2). Pathology (A5): clear cell renal cell carcinoma, 6.5 cm, ISUP grade 3, confined to the kidney, margins negative, 0 of 3 nodes. **Clear cell RCC diagnosed** |
| 2025-01-21 | practice | post-discharge visit; creatinine 2.2, stays CKD 3b (R5) |
| 2025-03-24 | outside imaging | CT chest without contrast: right-lung nodules larger (largest 9 to 14 mm), two new (A6) |
| 2025-04-08 | outside oncology | **metastatic clear cell RCC to the lungs** diagnosed on the imaging pattern (nodules too small and lung too fibrotic for safe biopsy); immunotherapy avoided because of the bleomycin lung; **sunitinib 50 mg daily, 4 weeks on and 2 off**, cycle 1 from 2025-05-05 (R4, A6) |
| 2025-05-20 | practice | follow-up (P) |
| 2025-08-11 | outside imaging | restaging CT chest: most nodules smaller or stable; one right lower lobe nodule larger, 14 to 21 mm (A7) |
| 2025-08-20 | outside oncology (S encounter 1) | returns after finishing the 2nd course (cycle) of sunitinib (R4) for metastatic RCC; history of bleomycin lung. Serum calcium 10.8 mg/dL. Hemoglobin 18 g/dL. CT-guided biopsy of the right lower lobe nodule performed the same day (S "a biopsy is performed", A7) |
| 2025-08-25 | outside pathology | biopsy: metastatic clear cell renal cell carcinoma (A7). Oncology continues sunitinib (cycles 3 to 5, through 2026-01-04) |
| 2025-09-09 | practice | follow-up: calcium 10.6, hemoglobin 17.6 (R9) |
| 2025-12-09 | practice | follow-up (P) |
| 2026-01-26 | outside imaging | CT chest without contrast: new 3.4 cm spiculated right upper lobe mass; other nodules stable (A8) |
| 2026-02-16 | outside oncology (S encounter 2) | age 64. Recently finished a course of sunitinib (cycle 5); visibly fatigued. Discussion of next steps for the **suspected lung malignancy** (new primary versus RCC metastasis). He says he knows the cancer is serious, his wife wants him to try every new treatment, and he admits he is very tired; she interrupts and urges him not to give up. Oncology **holds sunitinib** while they decide, offers a biopsy, refers to palliative care (A9) |
| 2026-03-03 | practice | visit: advance care planning; wife named health care proxy (A10). Sunitinib off the practice list |
| 2026-03-09 | outside palliative care | consult (A9): fatigue and breathlessness, goals discussed; no new drug |
| 2026-03-18 | outside | CT-guided biopsy, right upper lobe mass (A9); pathology 2026-03-23: metastatic clear cell renal cell carcinoma, not a new lung primary. Suspected lung malignancy resolved |
| 2026-03-30 | outside oncology | second-line therapy offered; he chooses a treatment break with CT in 3 months (A9) |
| 2026-04-21 | practice | telehealth follow-up (P) |
| 2026-06-22 | outside imaging | CT chest: right upper lobe mass 3.9 cm (was 3.4), other nodules slightly larger (A9) |
| 2026-06-29 | outside oncology | still off therapy; he wants more time to decide with his family; next visit in August (A9) |
| 2026-07-07 | practice | **last seen**: follow-up; BMP and CBC drawn and resulted (P) |
| 2026-08-10 | outside palliative care | follow-up letter, filed as received (A9): stable, still on a treatment break |
| after 2026-08-27 | practice | booked follow-up visit 2026-09-22 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** "Quit 2 years ago" at age 63 (August 2024). Resolution: he still smokes when he
  joins in May 2022 and quits in August 2022. Later notes say "quit August 2022".
- **R2.** The source's surgical years (lobectomy 2019, orchiectomy 2022, nephrectomy 2023)
  cannot hold: encounter 0 already has him six months into chemotherapy for a newly
  diagnosed testicular cancer. Resolution: re-dated with the timeline to lobectomy January
  2023, orchiectomy January 2024 and nephrectomy January 2025.
- **R3.** Acetaminophen 650 mg every 6 hours as needed is on every source list with no
  start. Resolution: started at the lobectomy discharge and kept as needed; NSAIDs avoided
  once his kidney function falls.
- **R4.** The source calls every systemic treatment "chemotherapy". Resolution: for the
  germ cell tumor it is BEP, 3 cycles (2024-02-12 to 2024-04-15), given in the oncology
  infusion unit; encounter 0's present-tense "takes combination chemotherapy" means the
  completed course. For the RCC it is sunitinib in 6-week courses (cycles); "2nd course"
  at encounter 1 is cycle 2 (2025-06-16 to 2025-07-27), and the course "recently
  completed" at encounter 2 is cycle 5. Sunitinib is prescribed by oncology; the practice
  lists it as a medication from oncology from 2025-05-20 to 2026-03-03.
- **R5.** CKD stage 3b is on the profile from the start. Resolution: creatinine about
  1.15 (eGFR about 70) from 2022 until chemotherapy; 1.8 in June 2024 after cisplatin
  (stage 3b diagnosed then); about 2.2 after the nephrectomy (eGFR about 32), still 3b.
- **R6.** Furosemide 20 mg daily has no start or reason in the source. Resolution: started
  2024-06-11 for blood pressure above goal (148/90) with stage 3b CKD; his sulfa
  antibiotic rash is noted and judged no bar to a non-antibiotic sulfonamide. BP about
  132/80 after.
- **R7.** "A high-resolution CT scan of the chest is shown." Resolution: a written HRCT
  report (non-contrast) with the findings in the timeline. Every CT is without contrast
  because of his contrast allergy; the abdominal staging that needs contrast is MRI with
  gadolinium.
- **R8.** Prednisone and oxygen are on the home list at the visit that diagnoses the
  fibrosis. Resolution: both start at encounter 0. The walk-test desaturation supports the
  oxygen; "Dependence on supplemental oxygen" and "Iatrogenic immunosuppression" (the
  source comorbidity, from prednisone) go on the list at the next practice visit.
- **R9.** Encounter 1 gives calcium 10.8 and hemoglobin 18 once. Resolution: oncology reads
  them as likely paraneoplastic; practice draws after that run calcium 10.4 to 10.7 and
  hemoglobin 17.2 to 17.8.
- **R10.** The source lists every cancer and the CKD at every encounter, and stamps clear
  cell RCC "diagnosed" at encounter 1 while that same encounter already treats metastatic
  RCC. Resolution: each problem starts at the event that documents it (timeline). Clear
  cell RCC is diagnosed on the nephrectomy pathology; metastatic disease on imaging in April
  2025, confirmed by the encounter 1 biopsy.
- **R11.** Outside letters copy their own medication lists; the practice list follows the
  resolved course.
- **Left as the source has it:** no calcium, vitamin D or bone-density testing on long-term
  prednisone; no treatment of the calcium of 10.8 or the hemoglobin of 18 beyond oncology
  watching them; lisinopril stays 5 mg. Nobody comments on these.

## Additions (A): physician to confirm

- **A1.** The 2022-05-10 new-patient visit (4 years with the practice).
- **A2.** How the stage I NSCLC was found and treated: screening LDCT (1.9 cm LUL nodule,
  Lung-RADS 4B), PET-CT (SUVmax 6.2, no other uptake), navigational bronchoscopic biopsy
  (adenocarcinoma), lobectomy pathology (2.0 cm, 0/9 nodes, margins negative, pT1b N0,
  stage IA), and a normal 6-month surveillance CT.
- **A3.** How the testicular tumor was found: a painless lump for about 3 weeks, scrotal
  ultrasound 2.8 cm left intratesticular solid mass, AFP 412 before surgery, 188 three
  weeks after, 4 after BEP; orchiectomy pathology NSGCT 3.0 cm, confined, margin negative;
  staging CT without metastases (stage IS, which is why BEP).
- **A4.** PFT and walk-test numbers for encounter 0 (TLC 64%, DLCO 41%, nadir 86%).
- **A5.** How the renal cancer was found: germ-cell surveillance imaging in November 2024
  (6.3 cm left renal mass, four 4 to 9 mm right-lung nodules), urology consult, nephrectomy
  pathology as in the timeline.
- **A6.** Metastatic disease on the March 2025 CT and sunitinib as first-line therapy.
- **A7.** The August 2025 restaging CT and the biopsy result at encounter 1.
- **A8.** The January 2026 CT finding behind encounter 2's suspected lung malignancy.
- **A9.** The course after the source ends: sunitinib held at encounter 2, palliative care
  consult, biopsy showing RCC metastasis (suspicion of a new lung primary set aside),
  treatment break by his choice, June CT (3.9 cm), oncology and palliative letters. No new
  drug.
- **A10.** Advance care planning at the practice (wife as health care proxy, MOLST
  discussed, full treatment for now).
- **A11.** Weight: 88 kg (2022 to 2023), 84 kg (mid-2024), 80 kg after the nephrectomy,
  76 kg by February 2026, 75 kg in July 2026. Height 178 cm.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention. An
  abnormal finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity grade
  or a symptom, or changes the diagnosis or plan.
- Records from his prior practice: hypertension on lisinopril, the two allergies, a
  normal screening colonoscopy in 2019 (repeat in 10 years).
- Visits every 3 to 6 months once the cancers start; BMP or CMP and CBC at most visits;
  lipids, A1c, urinalysis yearly (normal). BP about 136/84 until June 2024, about 132/80
  after.
- Prevention: hepatitis C and HIV screening once (negative); PCV20 and Shingrix (2 doses)
  in 2022; Tdap in 2022; influenza each fall (standard dose; he is under 65 through
  fall 2025); COVID each fall; RSV in fall 2024 (chronic lung disease).
  PHQ-2 or PHQ-9 yearly, low scores (PHQ-9 no higher than 6 in 2026, from fatigue).
- Refills, pharmacy and prior-authorization traffic, oxygen supplier paperwork, portal
  messages about results, refills and scheduling, one cancelled visit.
- After 2026-07-07: portal messages, refills, outside letters filed as received; no change
  to the practice's lists.
- Booked visit 2026-09-22: routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002319 on 2026-07-07. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
