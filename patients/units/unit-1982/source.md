# synthetic_hospital patient 1982

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial/Private.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-03 (Fri) | outpatient | Endocrinology |
| 1 | 2020-05-15 | 2022-10-17 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2023-06-12 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2024-12-16 (Mon) | outpatient | Internal Medicine |
| 4 | 2023-07-15 | 2025-12-12 (Fri) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial/Private",
 "occupation": "Attorney",
 "alcohol_use": "Social (occasional)",
 "family_history": [
  "Father: Myocardial infarction at age 52",
  "Mother: Type 2 Diabetes Mellitus",
  "Paternal grandfather: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 packs per day (10 cigarettes/day), reduced from 1 pack per day",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension (10-year duration)",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Obesity (BMI 32 kg/m\u00b2)",
  "Sedentary lifestyle"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Diabetes Mellitus diagnostic confirmation", "Primary prevention of cardiovascular disease", "Hyperlipidemia", "Stable Angina Pectoris (Medical Management)"]
Comorbidities: ["Cardiovascular disease risk factors", "Metabolic syndrome", "Obesity", "Tobacco use disorder", "Long-distance running", "Hyperlipidemia", "Essential hypertension", "Prediabetes"]

## Encounter 0: source 2020-01-15, practice 2022-06-03, outpatient, Endocrinology

Chief complaint: Comes to the clinic for a routine checkup.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Comes to the clinic for a routine checkup.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (10-year duration)
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity (BMI 32 kg/m²)
- Sedentary lifestyle

He has a 10-year history of hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had type 2 diabetes mellitus and died of a myocardial infarction at the age of 60.

SOCIAL HISTORY:
Continues to smoke about a pack of cigarettes daily despite counseling.
and he admits to minimal physical activity and frequently consuming fast food due to work constraints.

REVIEW OF SYSTEMS:
The patient has no current symptoms,

VITAL SIGNS:
His BMI is 33 kg/m²,
Blood pressure is 138/86 mm Hg, heart rate is 76/min, respiratory rate is 14/min, and BMI remains 33 kg/m².

PHYSICAL EXAMINATION:
His physical examination is within normal limits.

LABORATORY DATA:
His fasting blood glucose is 98 mg/dL, and his hemoglobin A1c is 5.8%.
Laboratory testing reveals a total cholesterol of 198 mg/dL, HDL of 38 mg/dL, LDL of 124 mg/dL, and triglycerides of 176 mg/dL.

## Encounter 1: source 2020-05-15, practice 2022-10-17, outpatient, Internal Medicine

Chief complaint: Presents to the clinic for a routine check-up. He has no complaints.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine check-up. He has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetes Mellitus diagnostic confirmation (diagnosed 2020-01-15)
- Hypertension (10-year duration)
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Obesity (BMI 32 kg/m²)
- Sedentary lifestyle

His medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for myocardial infarction in his father at the age of 50

SOCIAL HISTORY:
Leads a sedentary lifestyle. He smokes half a pack of cigarettes per day and has a body mass index (BMI) of 32 kg/m^2

VITAL SIGNS:
His blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Fasting plasma glucose: 146 mg/dL
- Hemoglobin A1c: 7.2%
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 2: source 2021-01-15, practice 2023-06-12, outpatient, Internal Medicine

Chief complaint: Presents to the clinic for a routine health check-up. He has no complaints.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine health check-up. He has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetes Mellitus diagnostic confirmation (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-05-15)
- Hypertension (10-year duration)
- Type 2 Diabetes Mellitus
- Obesity (BMI 32 kg/m²)
- Sedentary lifestyle

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
, controlled with lisinopril.

Home Medications:
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 52
- Mother: Type 2 Diabetes Mellitus
- Paternal grandfather: Hypertension

SOCIAL HISTORY:
Leads a sedentary lifestyle.
He smokes half a pack of cigarettes per day

VITAL SIGNS:
He has a body mass index (BMI) of 30 kg/m^2. His blood pressure is 135/85 mmHg.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 200 mg/dL
- Fasting plasma glucose: 110 mg/dL

OTHER STUDIES:
As part of a cross-sectional study on cardiovascular risk factors in the community, the patient agrees to participate. The study aims to estimate the prevalence of cardiovascular disease and its associated risk factors. Based on the patient's profile,

## Encounter 3: source 2022-07-15, practice 2024-12-16, outpatient, Internal Medicine

Chief complaint: Routine health examination. He reports occasional chest pain that he attributes to indigestion and denies any illicit drug use when asked.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination. He reports occasional chest pain that he attributes to indigestion and denies any illicit drug use when asked.

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of hypertension and diabetes. He reports occasional chest pain that he attributes to indigestion and denies any illicit drug use when asked.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetes Mellitus diagnostic confirmation (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-05-15)
- Hypertension (10-year duration)
- Type 2 Diabetes Mellitus
- Obesity (BMI 32 kg/m²)
- Sedentary lifestyle

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father, who had a myocardial infarction at age 50.

SOCIAL HISTORY:
He is a lawyer and admits to working long hours with high levels of stress. He smokes a pack of cigarettes daily and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 140/90 mmHg, heart rate is 85 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His laboratory values are as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 150 mg/dL
- Triglycerides: 175 mg/dL
- Fasting glucose: 110 mg/dL

OTHER STUDIES:
An electrocardiogram shows no abnormalities.

ASSESSMENT AND PLAN:
Given the patient's risk factors and presentation,

## Encounter 4: source 2023-07-15, practice 2025-12-12, outpatient, Cardiology

Chief complaint: 2-week history of intermittent chest pain. He describes a pressure-like sensation in the center of his chest that occurs primarily at rest and lasts for about 15 minutes each episode.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Amara Okafor, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of intermittent chest pain. He describes a pressure-like sensation in the center of his chest that occurs primarily at rest and lasts for about 15 minutes each episode.

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with hypertension, T2DM, and hyperlipidemia returns for evaluation of chest pain previously attributed to indigestion in 2022. He presents to the clinic with a 2-week history of intermittent chest pain that occurs primarily at rest and lasts for about 15 minutes each episode. He describes the pain as a pressure-like sensation in the center of his chest that does not radiate.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetes Mellitus diagnostic confirmation (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-05-15)
- Primary prevention of cardiovascular disease (diagnosed 2022-07-15)
- Hypertension (10-year duration)
- Type 2 Diabetes Mellitus
- Obesity (BMI 32 kg/m²)
- Sedentary lifestyle

He has a history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes for which he takes lisinopril and atorvastatin, respectively.

Home Medications:
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
He has a family history of coronary artery disease

SOCIAL HISTORY:
He smokes half a pack of cigarettes per day

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His laboratory results are as follows:

- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 150 mg/dL
- Troponin I: <0.04 ng/mL (normal)

OTHER STUDIES:
An electrocardiogram (ECG) shows no acute changes.

PLAN:
Given the clinical uncertainty, you discuss the potential seriousness of his symptoms and the need for further evaluation.
