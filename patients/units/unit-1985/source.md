# synthetic_hospital patient 1985

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Private PPO through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-30 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-07-01 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2022-12-26 (Mon) | outpatient | Internal Medicine |
| 3 | 2024-01-15 | 2025-12-22 (Mon) | outpatient | Internal Medicine |
| 4 | 2024-03-15 | 2026-02-25 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO through employer",
 "occupation": "High school history teacher",
 "alcohol_use": "Social, 1-2 drinks on weekends",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus diagnosed age 58",
  "Mother: Hypertension and hyperlipidemia",
  "Maternal grandfather: Myocardial infarction at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Hypokalemia managed with oral replacement (tolerating PO intake)", "Acute poststreptococcal glomerulonephritis", "Mixed hyperlipidemia", "Type 2 diabetes mellitus"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-12-30, outpatient, Internal Medicine

Chief complaint: Routine health examination for ongoing diabetes management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination for ongoing diabetes management

HISTORY OF PRESENT ILLNESS:
He has no complaints and feels generally well.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus

He has a history of hypertension.

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for type 2 diabetes in his father.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
On examination, his blood pressure is 132/85 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Fasting plasma glucose: 126 mg/dL
- Hemoglobin A1c: 6.5%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-07-15, practice 2022-07-01, outpatient, Internal Medicine

Chief complaint: Routine follow-up for hyperlipidemia management and medication adherence

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for hyperlipidemia management and medication adherence

HISTORY OF PRESENT ILLNESS:
The patient returns for hyperlipidemia management in the setting of known hypertension and type 2 diabetes mellitus, following his January 2020 encounter for diabetes care. He reports no new symptoms and is adherent to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Hypertension

MEDICATIONS:
Which includes metformin and lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus diagnosed age 58
- Mother: Hypertension and hyperlipidemia
- Maternal grandfather: Myocardial infarction at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks on weekends. Occupation: High school history teacher.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting plasma glucose: 154 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL

ASSESSMENT AND PLAN:
The patient's physician wants to assess the effectiveness of a new cholesterol-lowering medication. A study is conducted involving patients with a similar profile to the patient, and the 95% confidence interval for the mean reduction in LDL cholesterol is reported as (18 mg/dL, 26 mg/dL). Based on this information,

## Encounter 2: source 2021-01-15, practice 2022-12-26, outpatient, Internal Medicine

Chief complaint: Routine follow-up and inquiry about clinical trial participation for hyperlipidemia treatment

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up and inquiry about clinical trial participation for hyperlipidemia treatment

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old male with hypertension and type 2 diabetes mellitus who returns for continued longitudinal care following previous encounters for diabetes and hyperlipidemia management. He reports no new symptoms and feels generally well. The patient is interested in participating in a clinical trial he heard about that is investigating a new drug for the management of hyperlipidemia. The study is comparing the new drug to a placebo. He asks for your opinion on whether the study is worth considering.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Hypertension

MEDICATIONS:
He has been compliant with his medications, which include metformin and lisinopril.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus diagnosed age 58
- Mother: Hypertension and hyperlipidemia
- Maternal grandfather: Myocardial infarction at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks on weekends. Occupation: High school history teacher.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 154 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL

ASSESSMENT AND PLAN:
Upon reviewing the study details, you note that the planned sample size is relatively small. Given the patient's interest in the clinical trial and the information provided about the study,

## Encounter 3: source 2024-01-15, practice 2025-12-22, outpatient, Internal Medicine

Chief complaint: Two-week history of fatigue, decreased urine output, and generalized swelling with dark brown urine

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Okonkwo, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of fatigue, decreased urine output, and generalized swelling with dark brown urine

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with hypertension and type 2 diabetes followed in this clinic presents to clinic with a 2-week history of fatigue, decreased urine output, and generalized swelling. He reports noticing his urine has turned dark brown.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Hypertension

His past medical history is significant for type 2 diabetes mellitus and hypertension, which are managed with metformin and lisinopril, respectively.

MEDICATIONS:
Metformin and lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus diagnosed age 58
- Mother: Hypertension and hyperlipidemia
- Maternal grandfather: Myocardial infarction at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks on weekends. Occupation: High school history teacher.

VITAL SIGNS:
On physical examination, his blood pressure is 160/100 mmHg

PHYSICAL EXAMINATION:
He has 2+ pitting edema in his lower extremities.

LABORATORY DATA:
Urinalysis reveals brown urine with red blood cell casts and 3+ proteinuria. Laboratory results are as follows:

- Serum creatinine: 2.8 mg/dL (increased from a baseline of 1.0 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL
- Serum albumin: 3.2 g/dL
- Serum C3: Low
- Serum C4: Normal

## Encounter 4: source 2024-03-15, practice 2026-02-25, ed, Emergency Medicine

Chief complaint: Acute onset weakness and palpitations beginning 2 hours prior to arrival

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset weakness and palpitations beginning 2 hours prior to arrival

HISTORY OF PRESENT ILLNESS:
Patient with hypertension and type 2 diabetes presents to the emergency department with weakness and palpitations that started 2 hours ago following recent acute poststreptococcal glomerulonephritis. He denies vomiting, diarrhea, or laxative use.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Acute poststreptococcal glomerulonephritis (diagnosed 2024-01-15)
- Hypertension

MEDICATIONS:
He reports that he has been compliant with his medications, which include metformin, lisinopril, and a newly prescribed thiazide diuretic started one week ago.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus diagnosed age 58
- Mother: Hypertension and hyperlipidemia
- Maternal grandfather: Myocardial infarction at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks on weekends. Occupation: High school history teacher.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 102 bpm

PHYSICAL EXAMINATION:
Mild tremors

LABORATORY DATA:
Laboratory values are as follows:

- Serum potassium: 2.9 mEq/L
- Serum sodium: 140 mEq/L
- Serum chloride: 100 mEq/L
- Serum bicarbonate: 28 mEq/L
- Blood glucose: 180 mg/dL
- Serum creatinine: 1.0 mg/dL

OTHER STUDIES:
An ECG shows U waves and flattened T waves.
