# synthetic_hospital patient 2870

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Hispanic/Latina, Medicare with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-19 (Thu) | outpatient | Cardiology |
| 1 | 2020-09-15 | 2026-02-18 (Wed) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Blue Cross Blue Shield",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Ischemic stroke at age 70",
  "Sister: Rheumatic heart disease"
 ],
 "race_ethnicity": "Hispanic/Latina",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "0.125 mg oral daily",
   "name": "Digoxin"
  },
  {
   "dose": "5 mg oral daily (adjusted to INR 2.5-3.5)",
   "name": "Warfarin"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Mechanical mitral valve replacement (2015)",
  "Cataract extraction with intraocular lens placement (2020)"
 ],
 "chronic_conditions": [
  "Rheumatic heart disease",
  "Heart failure with reduced ejection fraction",
  "Chronic atrial fibrillation",
  "Hypertension",
  "Mechanical mitral valve (status post replacement)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Prosthetic mitral valve dysfunction"]
Comorbidities: ["Acute decompensated heart failure"]

## Encounter 0: source 2020-01-15, practice 2025-06-19, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and orthopnea over 2 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and orthopnea over 2 months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 2-month history of progressive dyspnea on exertion and orthopnea

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic heart disease
- Heart failure with reduced ejection fraction
- Chronic atrial fibrillation
- Hypertension
- Mechanical mitral valve (status post replacement)

PAST SURGICAL HISTORY:
She underwent mechanical mitral valve replacement 5 years ago

MEDICATIONS:
Home Medications:
- Digoxin 0.125 mg oral daily
- Warfarin 5 mg oral daily (adjusted to INR 2.5-3.5)
- Furosemide 40 mg oral daily
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 20 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Ischemic stroke at age 70
- Sister: Rheumatic heart disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is 78 bpm, and she has a 2+ pitting edema in her lower extremities. Auscultation of the heart reveals a high-pitched blowing murmur heard best at the apex, radiating to the axilla. The murmur is most prominent during the systolic phase of the cardiac cycle.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 12.5 g/dL
- White blood cell count: 7.2 x 10^3/uL
- Platelet count: 250 x 10^3/uL
- Serum creatinine: 1.1 mg/dL
- B-type natriuretic peptide (BNP): 450 pg/mL (elevated)

OTHER STUDIES:
An ECG shows normal sinus rhythm with no significant ST-T wave changes.

## Encounter 1: source 2020-09-15, practice 2026-02-18, inpatient, Internal Medicine

Chief complaint: Acute heart failure exacerbation requiring admission; subsequent development of nausea, blurred vision, and severe bradycardia on hospital day 3

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Acute heart failure exacerbation requiring admission; subsequent development of nausea, blurred vision, and severe bradycardia on hospital day 3

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with rheumatic heart disease and mechanical mitral valve, previously evaluated for prosthetic valve dysfunction, is hospitalized for acute heart failure exacerbation. On hospital day 3, she is found to have nausea, blurred vision, and a heart rate of 45/min.

PAST MEDICAL HISTORY:
Active Problem List:
- Prosthetic mitral valve dysfunction (diagnosed 2020-01-15)
- Rheumatic heart disease
- Heart failure with reduced ejection fraction
- Chronic atrial fibrillation
- Hypertension
- Mechanical mitral valve (status post replacement)

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (2015)
- Cataract extraction with intraocular lens placement (2020)

MEDICATIONS:
Review of her medications reveals she inadvertently received a double dose of digoxin due to a transcription error during the transition from the emergency department to the inpatient unit.

Home Medications:
- Warfarin 5 mg oral daily (adjusted to INR 2.5-3.5)
- Furosemide 40 mg oral daily
- Lisinopril 10 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 20 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Ischemic stroke at age 70
- Sister: Rheumatic heart disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

ASSESSMENT AND PLAN:
After stabilizing the patient, the hospital’s patient safety committee convenes a multidisciplinary meeting to evaluate how this error occurred. During their first session, they attempt to categorize potential contributing factors.
