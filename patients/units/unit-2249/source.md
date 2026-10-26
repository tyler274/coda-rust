# synthetic_hospital patient 2249

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-05-13 (Mon) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2019-08-16 (Fri) | outpatient | Cardiology |
| 2 | 2021-07-15 | 2020-11-05 (Thu) | inpatient | Pulmonology |
| 3 | 2022-01-15 | 2021-05-07 (Fri) | outpatient | Pulmonology |
| 4 | 2023-07-15 | 2022-10-30 (Sun) | inpatient | Infectious Disease |
| 5 | 2025-01-15 | 2024-05-13 (Mon) | outpatient | Oncology |
| 6 | 2026-07-15 | 2025-11-01 (Sat) | ed | Emergency Medicine |
| 7 | 2027-01-15 | 2026-05-16 (Sat) | icu | General Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "None",
 "family_history": [
  "Father: Prostate cancer (age 78)",
  "Mother: Hypertension, stroke (age 82)",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 20 years ago, 10 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg (variable dose for target INR 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "75 mcg PO daily",
   "name": "Levothyroxine"
  }
 ],
 "surgical_history": [
  "No prior major surgeries"
 ],
 "chronic_conditions": [
  "Atrial fibrillation (paroxysmal)",
  "Hypertension",
  "Hyperlipidemia",
  "Carotid artery stenosis",
  "Ischemic stroke with residual oropharyngeal dysphagia",
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Amiodarone-induced pulmonary toxicity", "Atrial fibrillation", "Metastatic carcinoma to bone", "Acute mesenteric ischemia", "Carotid artery stenosis", "Lung abscess (aspiration)", "Acute peripheral arterial thrombosis/embolism"]
Comorbidities: ["Atrial fibrillation", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-05-13, outpatient, Cardiology

Chief complaint: Routine follow-up after recent hospitalization for atrial fibrillation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up after recent hospitalization for atrial fibrillation

HISTORY OF PRESENT ILLNESS:
He was recently discharged from the hospital, where he was treated for an arrhythmia. At discharge, he was prescribed multiple medications, including one that requires regular laboratory monitoring due to its narrow therapeutic index and sensitivity to interactions with diet and other drugs. He has no complaints today. He has no bleeding, bruising, or shortness of breath. He is unsure which lab test is being used to monitor the medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Carotid artery stenosis
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

His past medical history includes hypertension, hyperlipidemia, and a prior lacunar stroke.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
He has been adherent to his medications, including the warfarin he started one week ago.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
Review of systems is unremarkable

PHYSICAL EXAMINATION:
Physical exam is within normal limits.

## Encounter 1: source 2020-04-15, practice 2019-08-16, outpatient, Cardiology

Chief complaint: Routine follow-up for carotid artery stenosis

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for carotid artery stenosis

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Carotid artery stenosis
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

He has a history of hypertension, hyperlipidemia, and carotid artery stenosis diagnosed two months ago during evaluation for transient dizziness.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
His medications include lisinopril, atorvastatin, and a medication recently prescribed by his neurologist to reduce his risk of stroke.

Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies chest pain, shortness of breath, focal weakness, or visual disturbances.

VITAL SIGNS:
His blood pressure today is 126/78 mmHg, and pulse is 78/min and regular.

PHYSICAL EXAMINATION:
Cardiovascular examination reveals normal heart sounds without murmurs. Neurologic examination shows no deficits. The remainder of his physical exam is unremarkable.

ASSESSMENT AND PLAN:
The medication prescribed to this patient reduces thrombotic risk by

## Encounter 2: source 2021-07-15, practice 2020-11-05, inpatient, Pulmonology

Chief complaint: 2-month history of progressive dyspnea and nonproductive cough

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
2-month history of progressive dyspnea and nonproductive cough

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with histories of paroxysmal atrial fibrillation and carotid artery stenosis was last evaluated by Cardiology in April 2020. He now presents for inpatient Pulmonology evaluation for progressive dyspnea and nonproductive cough, and he denies any fever, chills, or chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
He takes on amiodarone therapy for the past 18 months.

Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the bases. The rest of the examination is unremarkable.

LABORATORY DATA:
Normal complete blood count and metabolic panel

IMAGING:
A chest radiograph reveals bilateral reticular and ground-glass opacities. High-resolution computed tomography (HRCT) of the chest confirms the presence of diffuse interstitial changes.

## Encounter 3: source 2022-01-15, practice 2021-05-07, outpatient, Pulmonology

Chief complaint: Gradually worsening shortness of breath and persistent dry cough over several months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath and persistent dry cough over several months

HISTORY OF PRESENT ILLNESS:
The patient is a 74-year-old male with paroxysmal atrial fibrillation and prior hospitalization in July 2021 for dyspnea and cough who returns with recurrent respiratory symptoms. He reports feeling increasingly fatigued during his daily walks and recently had to stop midway when climbing a single flight of stairs. He also notes a persistent dry cough that is not relieved by over-the-counter medications. His symptoms have made it difficult for him to complete household chores or attend social gatherings.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2021-07-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

His past medical history includes paroxysmal atrial fibrillation, hyperlipidemia, and hypothyroidism.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
He does not take any over-the-counter supplements or herbal products.

Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
He has no history of recent travel or sick contacts, and he quit smoking over 20 years ago after a 10-pack-year history.

REVIEW OF SYSTEMS:
He denies chest pain, wheezing, fever, or sputum production.

PHYSICAL EXAMINATION:
On examination, his lungs are clear to auscultation without wheezes or crackles.

IMAGING:
Chest imaging reveals diffuse reticular opacities predominantly in the lower zones.

## Encounter 4: source 2023-07-15, practice 2022-10-30, inpatient, Infectious Disease

Chief complaint: Persistent fever, fatigue, and productive cough for 10 days with thick, malodorous sputum

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Johnson, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent fever, fatigue, and productive cough for 10 days with thick, malodorous sputum

HISTORY OF PRESENT ILLNESS:
Patient with dysphagia and prior pulmonary toxicity presents with worsening cough. Caregiver notes the cough has become increasingly thick and malodorous. He was febrile at home (39.1°C).

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2021-07-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

The patient has a history of an ischemic stroke two years ago, which left him with residual oropharyngeal dysphagia requiring a mechanical soft diet and occasional suctioning for secretions. His medical history also includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
He is afebrile today

PHYSICAL EXAMINATION:
Appears lethargic. On examination, decreased breath sounds are appreciated over the right upper chest, but there are no signs of consolidation or crackles.

LABORATORY DATA:
Labs reveal a leukocyte count of 16,400/mm³ with neutrophilic predominance. A sputum sample is thick, yellow-green, and foul-smelling. Gram stain of the sputum shows gram-negative rods, gram-positive cocci, and gram-positive rods.

IMAGING:
A chest x-ray is obtained and shown below.

PLAN:
The patient is started on IV fluids and supportive care.

## Encounter 5: source 2025-01-15, practice 2024-05-13, outpatient, Oncology

Chief complaint: Worsening lower back and pelvic pain with unintentional weight loss and decreased appetite

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Park, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening lower back and pelvic pain with unintentional weight loss and decreased appetite

HISTORY OF PRESENT ILLNESS:
This 77-year-old male with prior amiodarone-induced pulmonary toxicity and dysphagia presents with worsening pain. The pain is dull, progressive, and worse at night. He reports unintentional weight loss, decreased appetite, and new-onset fatigue. He has no bowel or bladder incontinence, lower extremity weakness, or sensory changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2023-07-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

His medical history includes hypertension and hyperlipidemia. He has not seen a physician in several years.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, there is tenderness to palpation over the lower lumbar spine and sacrum. Neurologic exam is normal.

LABORATORY DATA:
Hemoglobin                                 10.2 g/dL
ALP                                               245 U/L
PSA                                               89 ng/mL (normal <4)
Serum calcium                              9.3 mg/dL

IMAGING:
X-ray of the pelvis and lumbar spine shows multiple areas of increased sclerosis within the vertebral bodies and pelvis. A bone scan reveals increased uptake in multiple ribs, vertebrae, and the proximal femur.

## Encounter 6: source 2026-07-15, practice 2025-11-01, ed, Emergency Medicine

Chief complaint: Sudden onset of severe constant pain in left lower leg for 3 hours

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. James Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe constant pain in left lower leg for 3 hours

HISTORY OF PRESENT ILLNESS:
This 78-year-old male with paroxysmal atrial fibrillation, carotid stenosis, and metastatic bone disease presents with sudden severe constant left lower leg pain of three hours duration. He describes the pain as sudden in onset and constant. He denies any recent trauma or injury to the leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2023-07-15)
- Metastatic carcinoma to bone (diagnosed 2025-01-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
The patient&#x27;s current medications include warfarin and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On examination, the left lower leg is pale and cold to touch compared to the right. Pulses are not palpable in the left popliteal, dorsalis pedis, or posterior tibial arteries, while they are intact on the right side. Sensation is decreased in the left foot, and motor function is preserved but weak.

LABORATORY DATA:
Laboratory values are as follows:

- INR: 2.5
- Creatinine: 1.2 mg/dL
- Complete Blood Count: Within normal limits

OTHER STUDIES:
ECG shows atrial fibrillation with a controlled ventricular rate.

## Encounter 7: source 2027-01-15, practice 2026-05-16, icu, General Surgery

Chief complaint: Sudden-onset severe crampy left lower quadrant abdominal pain for 6 hours

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Aisha Patel, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset severe crampy left lower quadrant abdominal pain for 6 hours

HISTORY OF PRESENT ILLNESS:
This 79-year-old male with atrial fibrillation on warfarin and recent arterial thromboembolism presents with acute abdominal complaints. He presents to the emergency department with a 6-hour history of sudden-onset, crampy, left lower quadrant abdominal pain. He reports a recent history of intermittent, dull abdominal pain over the past few months, which he attributed to his "usual indigestion."

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-04-15)
- Amiodarone-induced pulmonary toxicity (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2023-07-15)
- Metastatic carcinoma to bone (diagnosed 2025-01-15)
- Acute peripheral arterial thrombosis/embolism (diagnosed 2026-07-15)
- Atrial fibrillation (paroxysmal)
- Hypertension
- Hyperlipidemia
- Ischemic stroke with residual oropharyngeal dysphagia
- Hypothyroidism

PAST SURGICAL HISTORY:
Endovascular abdominal aortic aneurysm repair 1 week ago

MEDICATIONS:
Home Medications:
- Warfarin 5 mg (variable dose for target INR 2.0-3.0)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO nightly
- Levothyroxine 75 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Prostate cancer (age 78)
- Mother: Hypertension, stroke (age 82)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker (quit 20 years ago, 10 pack-year history). Alcohol: None. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, and he is afebrile

PHYSICAL EXAMINATION:
Abdominal examination reveals mild distension, tenderness to palpation in the left lower quadrant without rebound or guarding, and no palpable masses. Rectal examination shows no masses or gross blood.

LABORATORY DATA:
White blood cell count of 12,000/uL, hemoglobin of 13.2 g/dL, and a normal basic metabolic panel

IMAGING:
CT scan of the abdomen and pelvis with intravenous contrast shows thickening of the sigmoid colon wall

OTHER STUDIES:
Colonoscopy reveals cyanotic mucosa and multiple non-bleeding, hemorrhagic ulcerations in the sigmoid colon
