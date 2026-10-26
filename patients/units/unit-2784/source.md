# synthetic_hospital patient 2784

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex F, Non-Hispanic Black, Medicare due to disability.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-17 (Tue) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2025-06-04 (Wed) | outpatient | Cardiology |
| 2 | 2021-04-15 | 2026-03-07 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (rash)"
 ],
 "insurance": "Medicare due to disability",
 "occupation": "Part-time library assistant",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother: Rheumatic heart disease",
  "Father: Hypertension, stroke at age 72",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg by mouth daily",
   "name": "Furosemide"
  },
  {
   "dose": "5 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg by mouth daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "5 mg by mouth twice daily",
   "name": "Apixaban"
  },
  {
   "dose": "20 mg by mouth at bedtime",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "History of rheumatic fever",
  "Atrial fibrillation",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Chronic mitral regurgitation"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-12-17, outpatient, Cardiology

Chief complaint: Progressive fatigue and exertional dyspnea over the past several months with orthopnea requiring pillow propping at night.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and exertional dyspnea over the past several months with orthopnea requiring pillow propping at night.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic due to progressive fatigue and exertional dyspnea over the past several months. She reports that she becomes short of breath when climbing stairs and must prop herself up with pillows at night to breathe comfortably.

PAST MEDICAL HISTORY:
Active Problem List:
- History of rheumatic fever
- Atrial fibrillation
- Hypertension
- Hyperlipidemia

She has a history of rheumatic fever in childhood but has not seen a physician in many years.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Furosemide 20 mg by mouth daily
- Lisinopril 5 mg by mouth daily
- Metoprolol succinate 25 mg by mouth daily
- Apixaban 5 mg by mouth twice daily
- Atorvastatin 20 mg by mouth at bedtime

ALLERGIES:
Penicillin (rash)

FAMILY HISTORY:
- Mother: Rheumatic heart disease
- Father: Hypertension, stroke at age 72
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She denies smoking, alcohol use, or drug use.

VITAL SIGNS:
Blood pressure is 124/72 mm Hg, pulse is 88/min, and respirations are 18/min.

PHYSICAL EXAMINATION:
Jugular venous pressure is not elevated. Bibasilar crackles are heard on lung auscultation. A murmur is heard on auscultation.

IMAGING:
Chest x-ray demonstrates pulmonary vascular congestion.

OTHER STUDIES:
ECG shows signs of left atrial enlargement.

## Encounter 1: source 2020-07-15, practice 2025-06-04, outpatient, Cardiology

Chief complaint: Worsening exertional dyspnea and fatigue over six months with recent onset of heart palpitations described as fluttering.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening exertional dyspnea and fatigue over six months with recent onset of heart palpitations described as fluttering.

HISTORY OF PRESENT ILLNESS:
Returning for mitral regurgitation follow-up with rheumatic history. She reports a recent onset of palpitations, which she describes as her heart "fluttering" in her chest.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic mitral regurgitation (diagnosed 2020-01-15)
- History of rheumatic fever
- Atrial fibrillation
- Hypertension
- Hyperlipidemia

Her past medical history is significant for rheumatic fever during childhood.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Furosemide 20 mg by mouth daily
- Lisinopril 5 mg by mouth daily
- Metoprolol succinate 25 mg by mouth daily
- Apixaban 5 mg by mouth twice daily
- Atorvastatin 20 mg by mouth at bedtime

ALLERGIES:
Penicillin (rash)

FAMILY HISTORY:
- Mother: Rheumatic heart disease
- Father: Hypertension, stroke at age 72
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Part-time library assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is irregularly irregular at 110 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a high-pitched, blowing holosystolic murmur heard best at the apex and radiating to the axilla. Lung examination is clear to auscultation bilaterally.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- Leukocyte count: 7,800 cells/mm^3
- Platelet count: 250,000 cells/mm^3
- Serum creatinine: 1.0 mg/dL
- B-type natriuretic peptide (BNP): 450 pg/mL (normal <100 pg/mL)

IMAGING:
A transthoracic echocardiogram demonstrates a dilated left atrium, mitral valve leaflet prolapse, and severe mitral regurgitation with a regurgitant fraction of 55%.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with no ST changes or Q waves.

## Encounter 2: source 2021-04-15, practice 2026-03-07, ed, Emergency Medicine

Chief complaint: Acute severe shortness of breath with paroxysmal nocturnal dyspnea and worsening orthopnea.

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute severe shortness of breath with paroxysmal nocturnal dyspnea and worsening orthopnea.

HISTORY OF PRESENT ILLNESS:
The patient with rheumatic fever returns following cardiology evaluations. She reports a 6-month history of progressive dyspnea on exertion and fatigue with recent onset of orthopnea and paroxysmal nocturnal dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic mitral regurgitation (diagnosed 2020-01-15)
- History of rheumatic fever
- Atrial fibrillation
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Furosemide 20 mg by mouth daily
- Lisinopril 5 mg by mouth daily
- Metoprolol succinate 25 mg by mouth daily
- Apixaban 5 mg by mouth twice daily
- Atorvastatin 20 mg by mouth at bedtime

ALLERGIES:
Penicillin (rash)

FAMILY HISTORY:
- Mother: Rheumatic heart disease
- Father: Hypertension, stroke at age 72
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Part-time library assistant.

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 92 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a holosystolic murmur at the apex radiating to the axilla and an S3 gallop. Lung examination is clear.

IMAGING:
An echocardiogram shows an enlarged left ventricle with an ejection fraction of 60%, severe mitral valve regurgitation, and a dilated left atrium.
