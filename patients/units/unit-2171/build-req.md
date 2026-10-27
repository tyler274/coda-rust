# Build requirements: unit-2171 (Howard Nadeau, MRN00002171)

Source: synthetic_hospital patient 2171, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who he is (S)

Howard Nadeau, born 1965-12-08, male, White, not Hispanic or Latino, English,
Northampton MA 01060, commercial PPO. Logistics coordinator. Former smoker, 20
pack-years, quit in 2017 (R4). Alcohol socially, 1 to 2 drinks a week. Marital status
married (identity field only; no spouse or other family member appears in the chart
beyond the family history).

- **Allergies:** penicillin (rash); shellfish (anaphylaxis).
- **Surgical history:** staging laparotomy with splenectomy, 2003 (Hodgkin staging);
  right hip arthroscopy, 2013; radical retropubic prostatectomy with bilateral pelvic
  lymphadenectomy, October 2020 (R2).
- **Family history:** father prostate cancer diagnosed at 72, deceased; mother breast
  cancer diagnosed at 68; maternal grandfather coronary artery disease.

**Joined the practice** 2022-06-14 (A1, new-patient visit; he moved his care from a
prior primary-care office). He brought:

- Problems: history of Hodgkin lymphoma, stage II, treated 2003 with mantle field
  radiation, in remission; prostate adenocarcinoma, Gleason 7 (3+4), status post
  radical prostatectomy (October 2020) and adjuvant external beam radiation with
  androgen deprivation (early 2021), followed by an outside urology group;
  hypertension; hyperlipidemia; osteopenia secondary to androgen deprivation therapy.
- Drugs: leuprolide 7.5 mg IM every 28 days (given at urology); amlodipine 5 mg daily;
  atorvastatin 20 mg daily; calcium carbonate 600 mg / vitamin D3 400 IU twice daily;
  aspirin 81 mg daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-06-14 | practice | new-patient visit (A1): history as above; BP controlled on amlodipine; epinephrine auto-injector prescribed for the shellfish anaphylaxis (A2); prior-office records requested |
| 2022-06-28 | outside records | prior primary-care records arrive: DXA 2022-02 with osteopenia (A3), colonoscopy 2019 normal, vaccines given around the 2003 splenectomy (P) |
| 2022-08-02 | outside | urology letter: on leuprolide, PSA <0.1 ng/mL (A4) |
| 2023-03-07 | outside | urology letter: PSA <0.1 ng/mL (A4) |
| 2023-05-18 | practice | annual physical: mild ankle swelling at day's end for a few weeks, attributed to amlodipine; no change made (R5) |
| 2023-09-21 | practice | about 6 months of increasing abdominal girth, leg swelling and dyspnea on exertion; edema on exam; urgent cardiology referral (A5) |
| 2023-10-09 | outside | cardiology consult letter: 6 months of increasing abdominal girth, lower-extremity swelling and dyspnea on exertion; no chest pain, palpitations or syncope. BP 115/75, HR 88, RR 18. Elevated JVP, pericardial knock, distended abdomen with fluid wave (ascites), liver edge 3 cm below the right costal margin, 2+ pitting edema to both knees. Hgb 13.2, WBC 7.8, platelets 235, creatinine 1.1, albumin 3.5, ALT 45, AST 50, total bilirubin 1.2, BNP 150. Chest radiograph: pericardial calcifications. Echo: thickened pericardium, normal systolic function, impaired diastolic filling. **Constrictive pericarditis diagnosed** (radiation-associated, after mantle field radiation). **Furosemide 40 mg daily started** (A6); cardiac MRI and catheterization arranged (A6) |
| 2023-10-30 | outside | cardiac MRI (A6): pericardial thickening to 6 mm, septal bounce, LVEF 58%, no myocardial late enhancement |
| 2023-11-20 | outside | right and left heart catheterization (A6): RA 18, RVEDP 19, PCWP 19, LVEDP 20 mmHg with dip-and-plateau and discordant respiratory variation; no obstructive coronary disease |
| 2024-01-16 to 2024-01-23 | outside | admission for radical pericardiectomy via sternotomy (A6); pathology fibrotic, calcified pericardium without malignancy. **Furosemide 40 stopped, 20 mg daily started** at discharge |
| 2024-02-08 | practice | post-discharge visit (P) |
| 2024-04-09 | outside | urology letter: PSA 0.2 ng/mL, low and detectable, recheck in 3 months (A4). No later urology letter reaches the practice (Records) |
| 2024-04-18 | outside | cardiology follow-up letter: edema and ascites resolved, BNP 62, ALT and AST normal; **furosemide stopped**; **constrictive pericarditis resolved** after pericardiectomy (A6) |
| 2024-06-13 | practice | annual physical; repeat DXA ordered (A3) |
| 2025-03-21 | practice (portal) | about a week of lower back pain; RN advises acetaminophen and books a visit for 2025-04-02 (R6) |
| 2025-04-01 | outside | ED record: 2 weeks of progressive lower back pain, difficulty walking and urinary incontinence; pins and needles in both legs; several episodes of incontinence in the past few days. Age 59 (R1). Increased lower-extremity tone, 3+ patellar and Achilles reflexes, Babinski sign positive on the left. Hgb 13.2, WBC 7.8, platelets 250, PSA 9.8 ng/mL (elevated), calcium 9.6, alkaline phosphatase 120 (elevated). Urgent MRI of the spine: mass compressing the thoracic spinal cord at T8. **Spinal metastasis with cord compression diagnosed** (R3). Admitted (A7) |
| 2025-04-02 | practice | the booked back-pain visit is cancelled (he is in hospital) |
| 2025-04-02 to 2025-04-09 | outside | admission (A7): dexamethasone in hospital; T8 decompressive laminectomy with epidural tumor resection and posterior instrumented fusion T6 to T10 on 2025-04-02; pathology metastatic adenocarcinoma, PSA and NKX3.1 positive, consistent with prostate primary; testosterone <20 ng/dL on leuprolide (castration-resistant). Incontinence resolved; discharged home walking with a cane; oxycodone-acetaminophen short course. Discharge summary reaches the practice late (2025-04-24) |
| 2025-04-29 | practice | post-discharge visit (P) |
| 2025-05-05 to 2025-05-16 | outside | post-operative radiation to T8, 30 Gy in 10 fractions (A7); completion letter |
| 2025-05-20 | outside | PSMA PET (A7): avid T8 lesion at the treated site, no other avid disease |
| 2025-06 | outside | oncology (A7) starts **enzalutamide 160 mg daily** and **denosumab 120 mg SC every 4 weeks**, leuprolide continued. No consult letter reaches the practice; the practice learns of enzalutamide from a pharmacy fill query and of denosumab from the patient (Records) |
| 2025-07-23 | practice | follow-up; enzalutamide and denosumab added to the list as reported (P) |
| 2025-08-19 | outside | oncology letter: PSA 1.2 ng/mL on enzalutamide (A7) |
| 2026-04-14 | outside | ED record: sudden severe lower back pain, bilateral leg weakness and acute urinary retention; pain constant and not relieved by over-the-counter analgesics; same kind of episode as the year before. Decreased light touch and pinprick below the umbilicus, 3/5 strength in both legs, diminished rectal tone. Hgb 13.2, WBC 7.8, platelets 250, creatinine 1.0, BUN 14, calcium 9.8. Urgent MRI of the spine: mass compressing the thoracic spinal cord (recurrent epidural disease at T8, R3). ED impression **cauda equina syndrome** (R3). Admitted (A8) |
| 2026-04-14 to 2026-04-24 | outside | admission (A8): dexamethasone in hospital; revision decompression at T8 on 2026-04-15, then stereotactic re-irradiation; urinary catheter |
| 2026-04-24 to 2026-05-08 | outside | inpatient rehabilitation (A8): catheter removed, voiding with post-void residual under 100 mL; walking with a rolling walker; strength 4/5 both legs at discharge. Summaries reach the practice 2026-05-12 |
| 2026-05-20 | practice | post-discharge visit (P) |
| 2026-06-09 | outside | oncology letter (A9): PSA 6.4 ng/mL (May 2026), progression on enzalutamide; **enzalutamide stopped**; **docetaxel 75 mg/m2 IV every 3 weeks with prednisone 5 mg twice daily started** 2026-06-09; leuprolide and denosumab continued |
| 2026-07-24 | practice | last visit (seed `last_seen`): follow-up during chemotherapy; CBC and CMP drawn and resulted (P) |
| 2026-08-18 | outside | oncology letter after cycle 4, filed as received; no list changes (Records) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The 2025 ED note calls him 58; on 2025-04-01 he is 59 by birth date. Ages
  follow the birth date. The source's "20 years prior", "3 years prior", "10 years
  ago" are fixed to calendar years (Hodgkin treatment and splenectomy 2003, hip
  arthroscopy 2013, prostatectomy 2020) and do not advance from note to note.
- **R2.** "Radical prostatectomy and adjuvant radiation 3 years prior" stands at every
  encounter. Resolution: prostatectomy October 2020, adjuvant external beam radiation
  with leuprolide started early 2021, leuprolide continued since. ADT this long for a
  Gleason 3+4 tumor is the urologist's choice and no practice note questions it.
- **R3.** The source names "cauda equina syndrome" yet both MRIs show thoracic cord
  compression, the 2025 exam shows upper motor neuron signs, and the 2026 note calls
  the 2025 event "prior cauda equina syndrome". Resolution: the 2025 event is a T8
  spinal metastasis with cord compression (the problem the practice lists). The 2026
  event is recurrent epidural disease at the same treated level; the ED records its
  impression as cauda equina syndrome because of retention and diminished rectal
  tone, and the hospital and practice carry that label as the ED gave it. No one
  remarks on the mismatch. "Prostate Cancer (Suspected)" is the 2025 ED's suspicion
  that the T8 mass is metastatic prostate cancer, confirmed by the 2025 pathology.
  "History of pelvic radiation therapy" is the 2021 adjuvant radiation.
- **R4.** "Quit 8 years ago" stands at every encounter. Resolution: quit in 2017.
- **R5.** A 6-month history in October 2023 starts in about April 2023. The May 2023
  physical notes only mild ankle swelling, put down to amlodipine. No drug change, no
  echo. That is the clinician's ordinary call and nobody revisits it.
- **R6.** The 2025 ED reports 2 weeks of back pain. Resolution: a portal message on
  2025-03-21 reports about a week of it; the RN suggests acetaminophen and books a
  visit; he goes to the ED before it. Nobody remarks on the cancer history in the
  reply.
- **R7.** Hemoglobin 13.2, WBC 7.8 and platelets 250 repeat in both ED records; they
  are carried as written. Constrictive pericarditis remains on outside problem lists
  in 2025 and 2026 after it resolved; outside lists copy forward.
- **Left as the source has it:** amlodipine and atorvastatin 20 mg stay unchanged
  throughout; aspirin continues. No one revisits the statin dose.

## Additions (A): physician to confirm

- **A1.** 2022-06-14 new-patient visit (four years with the practice; the source
  opens with a specialist consult).
- **A2.** Epinephrine 0.3 mg auto-injector for shellfish anaphylaxis, prescribed at
  joining.
- **A3.** DXA supporting the listed osteopenia: 2022-02 (prior office) lumbar spine
  T-score -1.4, femoral neck -1.8; 2024-06-27 (practice-ordered) lumbar spine -1.5,
  femoral neck -2.0.
- **A4.** Urology PSA letters: <0.1 (2022-08), <0.1 (2023-03), 0.2 (2024-04, recheck
  advised). The recheck letter never arrives.
- **A5.** 2023-09-21 practice visit that sends him to cardiology.
- **A6.** Constrictive pericarditis course: furosemide 40 mg daily from the consult;
  cardiac MRI and catheterization with the values in the timeline; radical
  pericardiectomy 2024-01-16; furosemide 20 mg at discharge, stopped 2024-04-18 with
  BNP 62 and normal ALT/AST; constriction resolved.
- **A7.** 2025 metastatic cord compression course: admission, T8 decompression and
  T6 to T10 fusion with pathology as in the timeline; castrate testosterone (<20)
  with PSA 9.8 makes it metastatic castration-resistant prostate cancer (the
  specific form of the source's prostate cancer and spinal metastasis); radiation
  30 Gy in 10 fractions; PSMA PET with only the T8 lesion; oncology starts
  enzalutamide 160 mg daily and denosumab 120 mg every 4 weeks; PSA 1.2 in August 2025.
  Oxycodone-acetaminophen 5/325 short course at discharge.
- **A8.** 2026 recurrence course: admission, revision T8 decompression, stereotactic
  re-irradiation, rehabilitation with the functional outcome in the timeline.
- **A9.** Progression: PSA 6.4 in May 2026; enzalutamide stopped; docetaxel 75 mg/m2
  every 3 weeks with prednisone 5 mg twice daily from 2026-06-09.
- **A10.** Lung cancer screening (20 pack-years, quit 2017): low-dose CT 2022-07,
  2023-07 and 2024-08, each Lung-RADS 1. The radiologist notes pericardial
  calcification on each, consistent with prior mantle radiation, and no one acts on it
  before October 2023.
- **A11.** Post-splenectomy vaccines: PCV20 (2022), MenACWY and the MenB series
  (2022 to 2023). Prior records say he had pneumococcal, Hib and meningococcal
  vaccines around 2003.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Blood pressure and lipid visits about every 6 months; BP runs about 124 to 132 over 78
  to 84 on amlodipine (115/75 at cardiology during constriction). LDL runs 95 to 110,
  HDL about 48, triglycerides about 130 on atorvastatin 20 mg. Creatinine 1.0 to 1.1.
  Yearly CMP, CBC, lipids; yearly TSH, normal (mantle radiation survivorship).
  Calcium and 25-hydroxy vitamin D normal. Hgb about 13.5 to 14.5 before 2025 and about
  12.5 to 13.2 after.
- Height 180 cm. Weight about 86 kg at joining, about 90 kg in late 2023 (fluid),
  about 84 kg after pericardiectomy, about 80 kg by 2026.
- Prevention: colonoscopy 2019 at the prior office, normal, repeat 2029; influenza and
  COVID vaccines each fall 2022 to 2025; Shingrix series 2022 to 2023; Tdap 2024;
  hepatitis C and HIV screening once (2022), negative; PHQ-2/PHQ-9 at physicals, low.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, records requests.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Chart with gaps (seed). The cardiology consult and follow-up, catheterization, MRI,
LDCT and ED records arrive. The 2024 pericardiectomy discharge summary arrives about
a week after discharge. The 2025 discharge summary arrives only after a records
request. The 2025 oncology consult letter never arrives: the practice learns of
enzalutamide from a pharmacy and of denosumab from the patient. The urology PSA
recheck after April 2024 never arrives. The 2026 hospital and rehabilitation
summaries arrive together on 2026-05-12.

## Last seen

The practice last saw MRN00002171 on 2026-07-24. Practice contacts after that day are
only messages and refills; outside records after it are filed as received without
changing the practice's lists. Nothing is booked after today.
