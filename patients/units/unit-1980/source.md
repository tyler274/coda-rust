# synthetic_hospital patient 1980

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 53 at first encounter, sex M, White, non-Hispanic, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-19 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-07-20 (Wed) | outpatient | Endocrinology |
| 2 | 2020-10-15 | 2023-01-06 (Fri) | outpatient | Internal Medicine |
| 3 | 2021-04-15 | 2023-07-17 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-04-15 | 2024-07-17 (Wed) | outpatient | Pulmonology |
| 5 | 2022-10-15 | 2025-01-03 (Fri) | outpatient | Pulmonology |
| 6 | 2023-10-15 | 2026-01-19 (Mon) | outpatient | Cardiology |
| 7 | 2024-04-15 | 2026-07-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Software developer",
 "alcohol_use": "Drinks 3-4 beers daily after work; high carbohydrate and high fat diet",
 "family_history": [
  "Father: Type 2 diabetes mellitus, hypertension",
  "Mother: Hypercholesterolemia, obesity",
  "Paternal grandfather: Myocardial infarction at age 62"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hypercholesterolemia",
  "Obesity (Class II)"
 ],
 "age_at_first_encounter": 53
}
```

Primary diagnoses: ["Hyperzincemia", "Type 2 diabetes mellitus", "Obesity hypoventilation syndrome", "Acute Gouty Arthritis (Podagra)"]
Comorbidities: ["Morbid obesity", "Alcohol consumption", "Metabolic syndrome", "Obesity", "Type 2 diabetes mellitus", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-04-19, outpatient, Internal Medicine

Chief complaint: Routine annual check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual check-up

HISTORY OF PRESENT ILLNESS:
A 52-year-old man comes to the clinic for a routine check-up. He is asymptomatic but reports inconsistent adherence to diet and medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

He has a history of poorly controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
His BMI is 33 kg/m² and blood pressure is 138/86 mm Hg.

LABORATORY DATA:
Laboratory studies are shown.
Total cholesterol				 210 mg/dL
LDL cholesterol				  130 mg/dL
HDL cholesterol				  35 mg/dL
Triglycerides					  650 mg/dL

## Encounter 1: source 2020-04-15, practice 2022-07-20, outpatient, Endocrinology

Chief complaint: Increased thirst, frequent urination, and unexplained weight loss

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased thirst, frequent urination, and unexplained weight loss

HISTORY OF PRESENT ILLNESS:
This 53-year-old male with T2DM returns following his January 2020 visit. He reports increased thirst, frequent urination, and recent unexplained weight loss of 10 pounds over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
He has a sedentary lifestyle and works as a software developer.

VITAL SIGNS:
His BMI is 32 kg/m^2, and his waist circumference is 102 cm. His blood pressure is 145/92 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals acanthosis nigricans around his neck.

LABORATORY DATA:
Fasting laboratory results are as follows:

- Fasting plasma glucose: 132 mg/dL
- Hemoglobin A1c: 7.2%
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL
- LDL cholesterol (calculated): 140 mg/dL

## Encounter 2: source 2020-10-15, practice 2023-01-06, outpatient, Internal Medicine

Chief complaint: Routine diabetes follow-up

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine diabetes follow-up

HISTORY OF PRESENT ILLNESS:
The patient returns for routine diabetes follow-up. He reports no major medical issues but mentions increased fatigue over the past few months. He denies polyuria, polydipsia, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
His BMI is 36 kg/m², blood pressure is 138/86 mm Hg, and pulse is 76/min.

PHYSICAL EXAMINATION:
Physical examination shows central obesity without acanthosis nigricans.

LABORATORY DATA:
A fasting blood glucose measured last week was 132 mg/dL, and HbA1c is 6.8%.

## Encounter 3: source 2021-04-15, practice 2023-07-17, outpatient, Internal Medicine

Chief complaint: Increased abdominal girth and fatigue

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Emily Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increased abdominal girth and fatigue

HISTORY OF PRESENT ILLNESS:
The patient is a 54-year-old male with Type 2 Diabetes Mellitus, hypertension, hypercholesterolemia, and obesity who returns for continued outpatient care. He reports a recent increase in abdominal girth and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
He admits to drinking 3-4 beers on most days after work. His diet is high in carbohydrates and fats.

REVIEW OF SYSTEMS:
He denies chest pain, palpitations, or dyspnea.

VITAL SIGNS:
His BMI is 32 kg/m^2, blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
He has mild hepatomegaly

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 130 mg/dL
- Triglycerides: 480 mg/dL
- HbA1c: 7.8%

## Encounter 4: source 2022-04-15, practice 2024-07-17, outpatient, Pulmonology

Chief complaint: Daytime sleepiness and morning headaches

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Daytime sleepiness and morning headaches

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
He has a BMI of 35 kg/m^2. On physical examination, his blood pressure is 145/90 mmHg, heart rate is 88 bpm, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
He has an enlarged neck circumference and exhibits pitting edema in his lower extremities.

LABORATORY DATA:
Arterial blood gas analysis on room air shows the following:

- pH: 7.43
- PaCO2: 55 mmHg
- PaO2: 68 mmHg
- HCO3^-: 32 mEq/L

A complete blood count reveals a hemoglobin level of 18.5 g/dL and hematocrit of 55%.

IMAGING:
An echocardiogram demonstrates right ventricular hypertrophy.

## Encounter 5: source 2022-10-15, practice 2025-01-03, outpatient, Pulmonology

Chief complaint: Worsening daytime sleepiness, morning headaches, and difficulty concentrating

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening daytime sleepiness, morning headaches, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
Male with T2DM, HTN, HLD, and Class II obesity returns. He reports his wife observed loud snoring, occasional gasping for air during sleep.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Obesity hypoventilation syndrome (diagnosed 2022-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
BMI is 42 kg/m^2, blood pressure is 145/92 mmHg, and heart rate is 88 bpm

PHYSICAL EXAMINATION:
His neck circumference is 44 cm, and he has pitting edema in his lower extremities.

LABORATORY DATA:
Arterial blood gas analysis on room air shows:

- pH: 7.32
- PaCO2: 58 mmHg
- PaO2: 68 mmHg
- HCO3^-: 29 mEq/L

IMAGING:
A chest X-ray is unremarkable.

OTHER STUDIES:
Pulmonary function tests reveal a mild restrictive pattern without significant reversibility after bronchodilator administration. Polysomnography confirms the presence of obstructive sleep apnea with an apnea-hypopnea index of 25 events per hour.

## Encounter 6: source 2023-10-15, practice 2026-01-19, outpatient, Cardiology

Chief complaint: Occasional chest discomfort on exertion

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Occasional chest discomfort on exertion

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with established Type 2 Diabetes Mellitus, obesity complicated by obesity hypoventilation syndrome, and recurrent hyperzincemia followed longitudinally since 2020 presents to Cardiology for evaluation of exertional symptoms. He reports no acute complaints but mentions occasional episodes of mild, non-radiating chest discomfort on exertion that resolves with rest. He denies any recent changes in his exercise tolerance, shortness of breath, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Obesity hypoventilation syndrome (diagnosed 2022-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
Cardiovascular examination reveals a regular rhythm without murmurs, rubs, or gallops.

LABORATORY DATA:
- Hemoglobin A1c: 7.2%
- Fasting Blood Glucose: 156 mg/dL
- Total Cholesterol: 210 mg/dL
- HDL Cholesterol: 40 mg/dL
- LDL Cholesterol: 130 mg/dL
- Triglycerides: 800 mg/dL

## Encounter 7: source 2024-04-15, practice 2026-07-11, ed, Emergency Medicine

Chief complaint: Severe pain in right big toe after wedding celebration

ENCOUNTER NOTE
Date: 2024-04-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in right big toe after wedding celebration

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with established Type 2 diabetes mellitus and Class II obesity who presents with acute foot pain. He describes the pain as sudden in onset, starting the night before and worsening to the point where he cannot bear weight on his foot. He reports that he attended a wedding two days ago where he consumed a significant amount of alcohol and had a steak dinner.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperzincemia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (diagnosed 2020-04-15)
- Obesity hypoventilation syndrome (diagnosed 2022-04-15)
- Hypertension
- Hypercholesterolemia
- Obesity (Class II)

His medical history includes hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, hypertension
- Mother: Hypercholesterolemia, obesity
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Drinks 3-4 beers daily after work; high carbohydrate and high fat diet. Occupation: Software developer.

VITAL SIGNS:
His temperature is 37.8°C (100°F).

PHYSICAL EXAMINATION:
On physical examination, the right first metatarsophalangeal joint is red, swollen, and exquisitely tender to touch.

LABORATORY DATA:
Laboratory values are as follows:

- Serum uric acid: 8.5 mg/dL (normal: 3.4-7.0 mg/dL)
- White blood cell count: 11,000/µL (normal: 4,500-11,000/µL)
- C-reactive protein: 20 mg/L (normal: <10 mg/L)
- Erythrocyte sedimentation rate: 45 mm/hr (normal: 0-20 mm/hr)

OTHER STUDIES:
Synovial fluid analysis from the affected joint shows negatively birefringent needle-shaped crystals under polarized light microscopy.
