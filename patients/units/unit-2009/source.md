# synthetic_hospital patient 2009

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex M, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-06 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-06-29 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2022-07-07 (Thu) | outpatient | Internal Medicine |
| 3 | 2023-01-15 | 2024-01-01 (Mon) | outpatient | Internal Medicine |
| 4 | 2025-01-15 | 2025-12-23 (Tue) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Office administrator",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father with myocardial infarction at age 55",
  "Mother with type 2 diabetes",
  "Paternal uncle with lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-year history; quit 5 years ago (previously quit 10 years ago after 20-year habit but relapsed for 5 years)",
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
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Obesity"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Hyperlipidemia", "Obesity", "Suspected malignant neoplasm of lung"]
Comorbidities: ["Family history of premature coronary artery disease", "History of recent pneumonia", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-01-06, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up. He has no complaints.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up. He has no complaints.

HISTORY OF PRESENT ILLNESS:
He has no complaints
The patient inquires about his risk of developing coronary artery disease (CAD) and whether he should make any changes to his lifestyle or medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Obesity

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is currently taking lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for coronary artery disease in his father, who had a myocardial infarction at age 55.

SOCIAL HISTORY:
He quit smoking 10 years ago after a 20-year pack-a-day habit.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, and his BMI is 27 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 110 mg/dL
- Triglycerides: 200 mg/dL
- Fasting glucose: 98 mg/dL

OTHER STUDIES:
A recent large cohort study reported the following odds ratios (OR) for various risk factors for CAD:

- Hypertension: OR 1.5 (95% CI 1.2-1.9)
- Hyperlipidemia: OR 2.0 (95% CI 1.7-2.4)
- Smoking: OR 3.0 (95% CI 2.5-3.6)
- Family history of CAD: OR 1.8 (95% CI 1.4-2.3)
- Obesity (BMI ≥30 kg/m^2): OR 1.2 (95% CI 0.9-1.6)

Based on this information,

## Encounter 1: source 2020-07-15, practice 2021-06-29, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine health examination. He has no complaints and feels generally well.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine health examination. He has no complaints and feels generally well.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his hypertension, hyperlipidemia, and obesity. He has no complaints and feels generally well.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes controlled with lisinopril.

Home Medications:
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for coronary artery disease in his father, who had a myocardial infarction at age 50.

SOCIAL HISTORY:
A 20-pack-year smoking history, although he quit smoking 5 years ago

VITAL SIGNS:
On physical examination, his blood pressure is 132/84 mmHg, heart rate is 78 beats per minute, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
His cardiovascular and respiratory examinations are unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 160 mg/dL
- Triglycerides: 125 mg/dL
- Fasting plasma glucose: 98 mg/dL

## Encounter 2: source 2021-07-15, practice 2022-07-07, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up. He has no complaints and has been adhering to his medication regimen.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up. He has no complaints and has been adhering to his medication regimen.

HISTORY OF PRESENT ILLNESS:
He returns for hyperlipidemia follow-up. He has no complaints and has been adhering to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-07-15)
- Hypertension

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
He has a family history of coronary artery disease

SOCIAL HISTORY:
He quit smoking 5 years ago

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His recent lipid panel is as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

PLAN:
The patient is interested in participating in a clinical trial he heard about that is investigating the efficacy of a new lipid-lowering agent compared to placebo. The study's primary endpoint is the reduction in LDL cholesterol levels after 12 months. He asks for your opinion on whether the study is worth considering.

## Encounter 3: source 2023-01-15, practice 2024-01-01, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up. He has no complaints.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up. He has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-07-15)
- Hypertension

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is currently taking lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for colorectal cancer in his father at the age of 60

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit smoking 10 years ago

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 110 mg/dL
- Triglycerides: 200 mg/dL
- Fasting glucose: 88 mg/dL
- Hemoglobin A1c: 5.4%

OTHER STUDIES:
His recent colonoscopy was normal

ASSESSMENT AND PLAN:
A recent case-control study was published in a medical journal assessing the association between statin use and the incidence of colorectal cancer. The study reported an adjusted odds ratio (OR) of 0.85 (95% CI, 0.65-1.10) for the development of colorectal cancer in patients taking statins compared to those not taking statins. Based on this information,

## Encounter 4: source 2025-01-15, practice 2025-12-23, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up. He has no complaints but mentions that several of his friends have recently been diagnosed with lung cancer, which has made him anxious about his own health.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up. He has no complaints but mentions that several of his friends have recently been diagnosed with lung cancer, which has made him anxious about his own health.

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up care, continuing long-term management of hypertension, hyperlipidemia, and obesity with demonstrated adherence to his established medication regimen. He has no complaints but mentions that several of his friends have recently been diagnosed with lung cancer, which has made him anxious about his own health. The patient inquires about his risk of developing lung cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (diagnosed 2020-01-15)
- Hyperlipidemia (diagnosed 2020-07-15)
- Hypertension

His past medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Lisinopril, atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with myocardial infarction at age 55
- Mother with type 2 diabetes
- Paternal uncle with lung cancer

SOCIAL HISTORY:
30-pack-year smoking history

REVIEW OF SYSTEMS:
He denies any cough, weight loss, or night sweats.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 beats per minute.

PHYSICAL EXAMINATION:
His lung examination is clear to auscultation bilaterally.

LABORATORY DATA:
Complete blood count: Within normal limits
- Lipid profile: Within normal limits
- Fasting glucose: Within normal limits

OTHER STUDIES:
You decide to conduct a case-control study to investigate the association between smoking and lung cancer. You collect data on smoking history from 100 lung cancer patients (cases) and 100 patients without lung cancer (controls) from the same community.
Based on the data collected, the odds of smoking in the lung cancer group (cases) is 70%, while in the non-lung cancer group (controls) it is 30%.
