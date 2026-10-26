# synthetic_hospital patient 2257

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-14 (Mon) | inpatient | Cardiology |
| 1 | 2020-09-15 | 2025-11-29 (Sat) | inpatient | Cardiology |
| 2 | 2021-03-15 | 2026-06-09 (Tue) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Hypertension",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 32 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement to right coronary artery"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["ST-segment elevation myocardial infarction, inferior wall", "Ventricular fibrillation (refractory)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-04-14, inpatient, Cardiology

Chief complaint: Sudden, intense pressure-like sensation in the center of his chest with fatigue and mild shortness of breath over the past two days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden, intense pressure-like sensation in the center of his chest with fatigue and mild shortness of breath over the past two days

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department after experiencing a sudden, intense pressure-like sensation in the center of his chest while watching television. He had been feeling generally fatigued and mildly short of breath over the past two days, but attributed it to age and weather.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to right coronary artery

MEDICATIONS:
His past medications include aspirin, a statin, and a beta blocker.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 32 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per month). Occupation: Retired postal worker.

REVIEW OF SYSTEMS:
The patient denies recent fever, cough, or leg swelling.

VITAL SIGNS:
Blood pressure of 92/58 mm Hg and heart rate of 68 bpm

PHYSICAL EXAMINATION:
On arrival, he appears anxious and slightly diaphoretic. Cardiopulmonary examination is unremarkable except for cool extremities and delayed capillary refill.

LABORATORY DATA:
Initial labs reveal a mild elevation in serum troponin.

OTHER STUDIES:
A 12-lead ECG demonstrates pronounced ST segment elevation in leads II, III, and aVF.

## Encounter 1: source 2020-09-15, practice 2025-11-29, inpatient, Cardiology

Chief complaint: Severe chest pain described as heavy pressure while mowing the lawn, radiating to left arm and jaw, with shortness of breath and diaphoresis

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe chest pain described as heavy pressure while mowing the lawn, radiating to left arm and jaw, with shortness of breath and diaphoresis

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with hypertension, hyperlipidemia, CAD who returns eight months after admission with recurrent symptoms. He describes severe chest pain that started 2 hours ago while he was mowing the lawn. The pain is described as a heavy pressure in the center of his chest, radiating to his left arm and jaw. He also reports shortness of breath and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to right coronary artery

MEDICATIONS:
Lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 32 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 98 bpm, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a fourth heart sound (S4)

LABORATORY DATA:
Serum cardiac markers are as follows:
- Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 50 U/L (normal 0-25 U/L)
- Myoglobin: 150 mcg/L (normal 28-72 mcg/L)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF

## Encounter 2: source 2021-03-15, practice 2026-06-09, icu, Cardiology

Chief complaint: Severe chest pain with pressure-like sensation radiating to left arm and jaw, complicated by refractory ventricular fibrillation

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Severe chest pain with pressure-like sensation radiating to left arm and jaw, complicated by refractory ventricular fibrillation

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with hypertension, hyperlipidemia, and coronary artery disease status post two prior ST-segment elevation myocardial infarctions presents to the emergency department with severe chest pain that started 45 minutes ago. The pain is described as a pressure-like sensation radiating to his left arm and jaw. Thirty minutes later, he suddenly becomes unresponsive, pulseless, and apneic.

PAST MEDICAL HISTORY:
Active Problem List:
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement to right coronary artery

MEDICATIONS:
His medications include lisinopril and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Hypertension
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 32 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per month). Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
He is diaphoretic and appears in distress.

LABORATORY DATA:
Troponin I: 24 ng/mL (Normal <0.04 ng/mL)
- Creatine kinase-MB: 55 U/L (Normal 0-24 U/L)
- Potassium: 4.2 mEq/L (Normal 3.5-5.0 mEq/L)
- Magnesium: 1.9 mg/dL (Normal 1.7-2.2 mg/dL)

OTHER STUDIES:
An ECG shows ST-segment elevation in leads II, III, and aVF.
The monitor shows a rapid, irregular, wide-complex rhythm without discernible QRS complexes.

PLAN:
The patient is given aspirin, nitroglycerin, and heparin.
