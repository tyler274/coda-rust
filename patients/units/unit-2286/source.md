# synthetic_hospital patient 2286

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 76 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-17 (Fri) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2024-09-06 (Fri) | outpatient | Cardiology |
| 2 | 2023-01-15 | 2026-03-13 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental Blue Shield",
 "occupation": "Retired high school teacher",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Type 2 diabetes mellitus, osteoporosis",
  "Older brother: Paget disease of bone",
  "Paternal grandfather: Stroke"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (30 pack-years, quit 1995)",
 "home_medications": [
  {
   "dose": "1000 mg orally twice daily",
   "name": "Metformin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg orally daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg orally as needed for arthritis pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Coronary artery disease",
  "Osteoarthritis",
  "Paget disease of bone",
  "Persistent atrial fibrillation"
 ],
 "age_at_first_encounter": 76
}
```

Primary diagnoses: ["Hyperosmolar hyperglycemic state", "Paget disease of bone", "Persistent or Drug-Refractory Atrial Fibrillation"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-03-17, outpatient, Internal Medicine

Chief complaint: Progressive difficulty hearing over the past year and sensation of fullness in right ear; recently required larger hat size due to tightness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty hearing over the past year and sensation of fullness in right ear; recently required larger hat size due to tightness

HISTORY OF PRESENT ILLNESS:
He also reports that he recently had to purchase a larger hat because his old one felt tight. He denies ear pain, discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Coronary artery disease
- Osteoarthritis
- Paget disease of bone
- Persistent atrial fibrillation

His past medical history includes type 2 diabetes mellitus and osteoarthritis.

PAST SURGICAL HISTORY:
He denies prior surgeries.

MEDICATIONS:
He takes metformin and acetaminophen as needed.

Home Medications:
- Aspirin 81 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, osteoporosis
- Older brother: Paget disease of bone
- Paternal grandfather: Stroke

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 1995). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school teacher.

REVIEW OF SYSTEMS:
The patient is otherwise asymptomatic and denies bone pain or recent fractures.

VITAL SIGNS:
His vitals are stable

PHYSICAL EXAMINATION:
On examination, there is bilateral frontal bossing and thickening of the skull bones. Neurologic exam reveals decreased hearing on the right but is otherwise normal. He has no focal tenderness or neurologic deficits.

LABORATORY DATA:
Laboratory evaluation shows normal serum calcium, phosphate, and parathyroid hormone levels. Serum alkaline phosphatase is markedly elevated.

IMAGING:
A lateral skull x-ray reveals thickened calvarium with areas of both lucency and sclerosis, producing a "cotton wool" appearance.

PLAN:
He is referred to endocrinology for further evaluation and counseling.

## Encounter 1: source 2021-07-15, practice 2024-09-06, outpatient, Cardiology

Chief complaint: Routine follow-up visit after hospitalization three weeks ago for palpitations and shortness of breath due to atrial fibrillation

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit after hospitalization three weeks ago for palpitations and shortness of breath due to atrial fibrillation

HISTORY OF PRESENT ILLNESS:
This 77-year-old male with persistent atrial fibrillation, coronary artery disease, and Paget disease of bone diagnosed in 2020 returns for outpatient follow-up. He was hospitalized 3 weeks ago for palpitations and shortness of breath and was found to have an irregularly irregular pulse. At that time, he was started on metoprolol and discharged in stable condition. He reports no ongoing symptoms and says his energy has improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Coronary artery disease
- Osteoarthritis
- Persistent atrial fibrillation

His medical history includes type 2 diabetes mellitus and coronary artery disease.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Current medications include aspirin, metoprolol, lisinopril, and atorvastatin.

Home Medications:
- Metformin 1000 mg orally twice daily
- Metoprolol succinate 50 mg orally daily
- Acetaminophen 500 mg orally as needed for arthritis pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, osteoporosis
- Older brother: Paget disease of bone
- Paternal grandfather: Stroke

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 1995). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school teacher.

VITAL SIGNS:
Vital signs are normal.

OTHER STUDIES:
ECG shows continued atrial fibrillation with a ventricular rate of 80/min.

## Encounter 2: source 2023-01-15, practice 2026-03-13, ed, Emergency Medicine

Chief complaint: Altered mental status and acute confusion over past two days accompanied by polyuria and polydipsia for one week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status and acute confusion over past two days accompanied by polyuria and polydipsia for one week

HISTORY OF PRESENT ILLNESS:
This 79-year-old male with Type 2 diabetes, coronary artery disease, persistent atrial fibrillation, and Paget disease (prior encounters 2020, 2021) presents with acute altered mental status. His daughter reports he has been increasingly confused over the past two days. She also mentions he has had polyuria and polydipsia for the past week. His diabetes has been managed with oral hypoglycemics, but he recently started prednisone for a severe case of poison ivy.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Persistent or Drug-Refractory Atrial Fibrillation (diagnosed 2021-07-15)
- Type 2 diabetes mellitus
- Coronary artery disease
- Osteoarthritis
- Persistent atrial fibrillation

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes oral hypoglycemics, prednisone.

Home Medications:
- Metformin 1000 mg orally twice daily
- Aspirin 81 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Acetaminophen 500 mg orally as needed for arthritis pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus, osteoporosis
- Older brother: Paget disease of bone
- Paternal grandfather: Stroke

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 1995). Alcohol: Occasional (1-2 drinks per month). Occupation: Retired high school teacher.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 160/90 mmHg, pulse is 110/min, and respirations are 22/min

PHYSICAL EXAMINATION:
On examination, the patient is lethargic but arousable, with dry mucous membranes and decreased skin turgor.

LABORATORY DATA:
Serum glucose: 800 mg/dL
- Serum sodium: 145 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum osmolality: 320 mOsm/kg
- Urine ketones: Negative
- Arterial pH: 7.35
