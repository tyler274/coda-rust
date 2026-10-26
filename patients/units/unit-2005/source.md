# synthetic_hospital patient 2005

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, African American, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-15 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-04-22 (Fri) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2023-04-21 (Fri) | outpatient | Cardiology |
| 3 | 2022-07-15 | 2024-04-15 (Mon) | outpatient | Cardiology |
| 4 | 2024-01-15 | 2025-10-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Postal Service Supervisor",
 "alcohol_use": "Social drinker; currently consumes one 5-oz glass of red wine daily with dinner",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 5 years ago",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "No prior abdominal surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Mixed hyperlipidemia",
  "Recurrent diverticulitis",
  "Hyperzincemia"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Mixed hyperlipidemia", "Hyperzincemia", "African American Ethnicity", "Perforated diverticulitis with peritonitis"]
Comorbidities: ["Essential hypertension", "Mixed hyperlipidemia"]

## Encounter 0: source 2020-01-15, practice 2021-10-15, outpatient, Internal Medicine

Chief complaint: Routine annual physical examination with questions about dietary modifications.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual physical examination with questions about dietary modifications.

HISTORY OF PRESENT ILLNESS:
He mentions that he has been reading about the health benefits of moderate alcohol consumption and has started drinking one glass of red wine with dinner each night. He asks if this is a good idea for his health.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus
- Mixed hyperlipidemia
- Recurrent diverticulitis
- Hyperzincemia

PAST SURGICAL HISTORY:
- No prior abdominal surgeries

MEDICATIONS:
He is currently taking hydrochlorothiazide for his blood pressure.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
His blood pressure today is 135/85 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His recent blood tests show:

- Total cholesterol: 195 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 175 mg/dL
- Fasting glucose: 98 mg/dL

ASSESSMENT AND PLAN:
You recall a recent study that showed a reduced risk of heart disease in moderate drinkers compared to nondrinkers. However, you are concerned about the potential confounding factors in the study.

## Encounter 1: source 2020-07-15, practice 2022-04-22, outpatient, Internal Medicine

Chief complaint: Follow-up visit for medication counseling regarding concerns about long-term statin therapy.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for medication counseling regarding concerns about long-term statin therapy.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of mixed hyperlipidemia and diabetes following his January 2020 annual examination. He reports no acute complaints but mentions he has been reading articles online about the risks of statin therapy and is considering stopping his atorvastatin. He is concerned about potential liver damage and the development of diabetes.

PAST MEDICAL HISTORY:
Active Problem List:
- African American Ethnicity (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Mixed hyperlipidemia
- Recurrent diverticulitis
- Hyperzincemia

PAST SURGICAL HISTORY:
- No prior abdominal surgeries

MEDICATIONS:
His current medications include atorvastatin 40 mg daily and lisinopril 20 mg daily.

Home Medications:
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Social drinker; currently consumes one 5-oz glass of red wine daily with dinner. Occupation: Postal Service Supervisor.

VITAL SIGNS:
His blood pressure is 132/84 mmHg.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL
- Fasting glucose: 98 mg/dL
- Liver function tests: within normal limits

## Encounter 2: source 2021-07-15, practice 2023-04-21, outpatient, Cardiology

Chief complaint: Routine cardiology consultation for comprehensive cardiovascular risk stratification.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiology consultation for comprehensive cardiovascular risk stratification.

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and diabetes returns following dietary and statin counseling. He is concerned about his risk of developing cardiovascular disease and asks what more he can do to reduce his risk.

PAST MEDICAL HISTORY:
Active Problem List:
- African American Ethnicity (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Recurrent diverticulitis
- Hyperzincemia

PAST SURGICAL HISTORY:
- No prior abdominal surgeries

MEDICATIONS:
He has been compliant with his medications, which include metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Social drinker; currently consumes one 5-oz glass of red wine daily with dinner. Occupation: Postal Service Supervisor.

REVIEW OF SYSTEMS:
He denies any chest pain, shortness of breath, or changes in vision.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His recent laboratory tests are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 154 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 200 mg/dL
- Serum creatinine: 1.0 mg/dL
- Urine albumin-to-creatinine ratio: 250 mg/g

## Encounter 3: source 2022-07-15, practice 2024-04-15, outpatient, Cardiology

Chief complaint: Request for information regarding eligibility for a clinical trial investigating novel lipid-lowering therapies.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Request for information regarding eligibility for a clinical trial investigating novel lipid-lowering therapies.

HISTORY OF PRESENT ILLNESS:
The patient returns to the Cardiology clinic for ongoing management of his cardiovascular risk factors, following prior cardiology consultation in 2021 for comprehensive risk stratification and longitudinal management of mixed hyperlipidemia. He reports no current symptoms and is adherent to his medications. The patient is interested in participating in a clinical trial he heard about, which is investigating the incidence of myocardial infarction (MI) in high-risk patients. The trial is comparing a new lipid-lowering agent against placebo. He asks for your opinion on whether he should participate. Based on the information provided,

PAST MEDICAL HISTORY:
Active Problem List:
- African American Ethnicity (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Recurrent diverticulitis
- Hyperzincemia

PAST SURGICAL HISTORY:
- No prior abdominal surgeries

MEDICATIONS:
He is currently taking lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL
- Fasting plasma glucose: 98 mg/dL

## Encounter 4: source 2024-01-15, practice 2025-10-11, ed, Emergency Medicine

Chief complaint: Sudden onset of severe, diffuse abdominal pain with nausea and fever following a meal.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe, diffuse abdominal pain with nausea and fever following a meal.

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with recurrent diverticulitis and longitudinal care since 2020. He presents with pain that began 4 hours ago after a meal. He describes the pain as initially sharp and intense, which then partially subsided for a brief period, but has now become more diffuse and constant. He also reports nausea and a low-grade fever.

PAST MEDICAL HISTORY:
Active Problem List:
- African American Ethnicity (diagnosed 2020-01-15)
- Mixed hyperlipidemia (diagnosed 2020-07-15)
- Hyperzincemia (diagnosed 2022-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Recurrent diverticulitis

His past medical history is significant for hypertension, for which he takes lisinopril, and recurrent diverticulitis.

PAST SURGICAL HISTORY:
- No prior abdominal surgeries

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Atorvastatin 40 mg daily
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: Social drinker; currently consumes one 5-oz glass of red wine daily with dinner. Occupation: Postal Service Supervisor.

VITAL SIGNS:
On examination, his temperature is 38.2°C (100.8°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals generalized tenderness, rebound tenderness, and guarding. Bowel sounds are diminished.

LABORATORY DATA:
Laboratory tests show leukocytosis with a left shift.

IMAGING:
An upright chest X-ray demonstrates free air under the diaphragm.
