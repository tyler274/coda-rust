# synthetic_hospital patient 2696

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Hispanic, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-19 (Thu) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-02-22 (Mon) | outpatient | Rheumatology |
| 2 | 2021-01-15 | 2021-11-18 (Thu) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2023-05-24 (Wed) | outpatient | Endocrinology |
| 4 | 2025-01-15 | 2025-11-22 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Administrative assistant",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Type 2 diabetes mellitus in mother",
  "Hypertension in father",
  "Cerebrovascular disease in maternal grandmother"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "15 mg PO daily",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Cushing syndrome",
  "Systemic lupus erythematosus"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Severe hypoglycemia", "Type 2 diabetes mellitus", "Acute ischemic stroke", "Long term (current) use of immunosuppressants", "Cushing syndrome"]
Comorbidities: ["Obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-11-19, outpatient, Endocrinology

Chief complaint: Increasing fatigue and difficulty climbing stairs

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue and difficulty climbing stairs

HISTORY OF PRESENT ILLNESS:
6-month history of increasing fatigue and difficulty climbing stairs. She reports that she has gained 20 pounds during this period despite no significant changes in her diet or exercise routine. She also mentions experiencing easy bruising and her menses have become irregular.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Cushing syndrome
- Systemic lupus erythematosus

Her medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Prednisone 15 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Hypertension in father
- Cerebrovascular disease in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note a rounded face, supraclavicular fat pads, and thinning of the skin. Muscle strength testing reveals 3/5 strength in the proximal muscles of the upper and lower extremities.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum cortisol (8 AM): 28 mcg/dL (normal: 6-23 mcg/dL)
- 24-hour urinary free cortisol: 350 mcg/24 hr (normal: 20-90 mcg/24 hr)
- ACTH: 22 pg/mL (normal: 9-52 pg/mL)
- Fasting glucose: 160 mg/dL
- HbA1c: 7.8%

## Encounter 1: source 2020-04-15, practice 2021-02-22, outpatient, Rheumatology

Chief complaint: Follow-up after being diagnosed with type II diabetes mellitus three months ago

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Aisha Patel, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up after being diagnosed with type II diabetes mellitus three months ago

HISTORY OF PRESENT ILLNESS:
This 45-year-old female with Cushing syndrome diagnosed three months ago in Endocrinology, systemic lupus erythematosus, and type 2 diabetes mellitus returns for follow-up regarding immunosuppressant use. She reports occasional dizziness when standing up quickly but denies chest pain, shortness of breath, or palpitations. Her home blood glucose logs show moderate control with metformin. Two weeks later, she returns with improved blood pressure readings but notes some lightheadedness when getting up too fast, which resolves within seconds. She has no complaints of swelling, chest pain, or fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Cushing syndrome (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Systemic lupus erythematosus

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes metformin.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Lisinopril 10 mg PO daily
- Prednisone 15 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Hypertension in father
- Cerebrovascular disease in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Administrative assistant.

VITAL SIGNS:
On today’s visit, her blood pressure is elevated at 152/96 mm Hg. Two weeks later, she returns with improved blood pressure readings. Her pulse is mildly elevated.

PHYSICAL EXAMINATION:
Physical exam shows no jugular venous distension, lung crackles, or peripheral edema

LABORATORY DATA:
Laboratory monitoring shows stable renal function.

IMAGING:
A transthoracic echocardiogram done as part of a baseline evaluation shows preserved systolic function with no wall motion abnormalities.

PLAN:
Primary care physician starts her on a new antihypertensive. The physician explains that the new medication works by interfering with a hormone that normally narrows blood vessels and that the drug also has protective benefits for the kidneys in diabetics.

## Encounter 2: source 2021-01-15, practice 2021-11-18, outpatient, Internal Medicine

Chief complaint: Routine follow-up

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Robert Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient returns for routine diabetes follow-up in the context of known Cushing syndrome and systemic lupus erythematosus. She has been managing her diabetes with metformin and a balanced diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Cushing syndrome (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Systemic lupus erythematosus

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Prednisone 15 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Hypertension in father
- Cerebrovascular disease in maternal grandmother

SOCIAL HISTORY:
She does not smoke and has a sedentary lifestyle.

REVIEW OF SYSTEMS:
She denies any chest pain, shortness of breath, or leg pain.

VITAL SIGNS:
Her blood pressure today is 132/86 mmHg. Her body mass index is 29 kg/m^2.

LABORATORY DATA:
Laboratory results are as follows:
- Hemoglobin A1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 3: source 2022-07-15, practice 2023-05-24, outpatient, Endocrinology

Chief complaint: Presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of Cushing syndrome and Type 2 diabetes mellitus. She reports adherence to her diet and exercise regimen, but she has been experiencing frequent episodes of diaphoresis and palpitations, particularly in the early mornings.

PAST MEDICAL HISTORY:
Active Problem List:
- Cushing syndrome (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2021-01-15)
- Hypertension
- Systemic lupus erythematosus

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Her current medications include metformin and insulin glargine.

Home Medications:
- Lisinopril 10 mg PO daily
- Prednisone 15 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Hypertension in father
- Cerebrovascular disease in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She denies any chest pain, dyspnea, or recent illnesses.

VITAL SIGNS:
Her blood pressure is 132/84 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 6.2%
- Fasting plasma glucose: 72 mg/dL
- Serum creatinine: 1.0 mg/dL
- Urinary albumin-to-creatinine ratio: 25 mg/g

## Encounter 4: source 2025-01-15, practice 2025-11-22, ed, Emergency Medicine

Chief complaint: Sudden onset of confusion and slurred speech that began 2 hours ago

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of confusion and slurred speech that began 2 hours ago

HISTORY OF PRESENT ILLNESS:
This 50-year-old female with Type 2 diabetes mellitus, hypertension, Cushing syndrome, and systemic lupus erythematosus on chronic prednisone therapy, who experienced severe hypoglycemia in 2022, is brought to the emergency department by her husband due to sudden onset confusion and slurred speech that began two hours ago. She has been compliant with her medications, which include metformin, insulin glargine, lisinopril, and prednisone. Her husband is eager to know her condition and asks for updates on her status. The patient's sister arrives and also requests information about her condition.

PAST MEDICAL HISTORY:
Active Problem List:
- Cushing syndrome (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Type 2 diabetes mellitus (diagnosed 2021-01-15)
- Severe hypoglycemia (diagnosed 2022-07-15)
- Hypertension
- Systemic lupus erythematosus

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes metformin and lisinopril.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Prednisone 15 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 2 diabetes mellitus in mother
- Hypertension in father
- Cerebrovascular disease in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 155/90 mmHg, pulse is 88/min, and temperature is 36.8°C (98.2°F).

PHYSICAL EXAMINATION:
On examination, the patient is alert but aphasic.
Neurological examination reveals right facial droop and right arm weakness.

LABORATORY DATA:
Her fasting blood glucose is 180 mg/dL and her HbA1c is 7.8%.

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.
