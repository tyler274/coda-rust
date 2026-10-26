# synthetic_hospital patient 1705

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-06 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-09-05 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-07-15 | 2022-12-05 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-01-15 | 2023-06-13 (Tue) | outpatient | Endocrinology |
| 4 | 2021-07-15 | 2023-11-30 (Thu) | outpatient | Psychiatry |
| 5 | 2022-01-15 | 2024-05-27 (Mon) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2024-11-26 (Tue) | outpatient | Neurology |
| 7 | 2024-01-15 | 2026-05-31 (Sun) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time administrative assistant",
 "alcohol_use": "Social, 1-2 drinks monthly",
 "family_history": [
  "Von Hippel-Lindau syndrome (father)",
  "Type 2 diabetes mellitus (mother)",
  "Bipolar disorder (maternal uncle)",
  "Hypertension (mother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Olanzapine"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Left radical nephrectomy for clear cell renal cell carcinoma (age 31)"
 ],
 "chronic_conditions": [
  "Bipolar I Disorder",
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Von Hippel-Lindau Syndrome",
  "Cushing Syndrome",
  "Polycystic Ovary Syndrome",
  "History of clear cell renal cell carcinoma"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Type 2 diabetes mellitus", "Hyperinsulinemia", "Cushing syndrome", "Polycystic ovary syndrome", "Von Hippel-Lindau syndrome", "Acute pyelonephritis", "Type 1 diabetes mellitus"]
Comorbidities: ["Clear cell renal cell carcinoma", "Pheochromocytoma", "Retinal angioma", "Obesity", "Obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-06-06, outpatient, Internal Medicine

Chief complaint: Polyuria, polydipsia, and unexplained weight loss for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Polyuria, polydipsia, and unexplained weight loss for 6 months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of polyuria, polydipsia, and unexplained weight loss. She recently obtained Medicaid coverage and decided to seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I Disorder
- Type 2 Diabetes Mellitus
- Hypertension
- Von Hippel-Lindau Syndrome
- Cushing Syndrome
- Polycystic Ovary Syndrome
- History of clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Home Medications:
- Olanzapine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for type 2 diabetes in her mother.

SOCIAL HISTORY:
She has not seen a doctor in several years due to lack of health insurance

VITAL SIGNS:
Her BMI is 22 kg/m^2, blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
She appears dehydrated

LABORATORY DATA:
A random blood glucose level is 380 mg/dL. Additional laboratory values are as follows: - Hemoglobin A1c: 11.2% - Serum creatinine: 1.0 mg/dL - Urinalysis: Glucose 3+, Ketones 1+ - Thyroid-stimulating hormone (TSH): 2.5 mIU/L

## Encounter 1: source 2020-04-15, practice 2022-09-05, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of irregular menstrual cycles and difficulty conceiving

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of irregular menstrual cycles and difficulty conceiving

HISTORY OF PRESENT ILLNESS:
The patient with polycystic ovary syndrome presents after recent Internal Medicine evaluation. She reports that her menstrual cycles have become increasingly unpredictable, with intervals ranging from 45 to 60 days, and she has experienced heavy bleeding with each cycle.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Bipolar I Disorder
- Type 2 Diabetes Mellitus
- Hypertension
- Von Hippel-Lindau Syndrome
- Cushing Syndrome
- Polycystic Ovary Syndrome
- History of clear cell renal cell carcinoma

She has a past medical history of type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Her current medications include metformin and lisinopril.

Home Medications:
- Olanzapine 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Von Hippel-Lindau syndrome (father)
- Type 2 diabetes mellitus (mother)
- Bipolar disorder (maternal uncle)
- Hypertension (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
She has a BMI of 34 kg/m^2. Her blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
On physical examination, you note central obesity, acanthosis nigricans, and hirsutism.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting glucose: 126 mg/dL
- Hemoglobin A1c: 7.2%
- Total testosterone: 45 ng/dL (normal range: 8-60 ng/dL)
- Sex hormone-binding globulin (SHBG): Low
- Estrone: Elevated

## Encounter 2: source 2020-07-15, practice 2022-12-05, outpatient, Internal Medicine

Chief complaint: Headaches and visual disturbances for the past 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Headaches and visual disturbances for the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient with VHL returns after GYN care. She presents to clinic with a complaint of headaches and visual disturbances for the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Bipolar I Disorder
- Type 2 Diabetes Mellitus
- Hypertension
- Von Hippel-Lindau Syndrome
- Cushing Syndrome
- History of clear cell renal cell carcinoma

Her medical history is significant for hypertension, which was diagnosed two years ago and is currently well-controlled with metoprolol (a beta blocker).

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
She takes metoprolol (a beta blocker).

Home Medications:
- Olanzapine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She mentions that her brother was recently diagnosed with a brain tumor.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Blood pressure is 142/90 mmHg

PHYSICAL EXAMINATION:
Fundoscopic examination reveals retinal angiomas

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 13.2 g/dL
- Hematocrit: 39.8%
- Serum calcium: 9.8 mg/dL
- Plasma metanephrines: Elevated

IMAGING:
An MRI of the brain shows multiple cerebellar and spinal hemangioblastomas. Abdominal ultrasound reveals bilateral renal cysts and a solid mass in the left kidney.

ASSESSMENT AND PLAN:
Given the patient's family history, clinical findings, and test results,

## Encounter 3: source 2021-01-15, practice 2023-06-13, outpatient, Endocrinology

Chief complaint: 6-month history of weight gain, easy bruising, and menstrual irregularities

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of weight gain, easy bruising, and menstrual irregularities

HISTORY OF PRESENT ILLNESS:
Patient with VHL, PCOS, and diabetes returns reporting increased facial roundness and a recent onset of acne.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Von Hippel-Lindau syndrome (diagnosed 2020-07-15)
- Bipolar I Disorder
- Type 2 Diabetes Mellitus
- Hypertension
- Cushing Syndrome
- History of clear cell renal cell carcinoma

Her past medical history is significant for hypertension and type 2 diabetes mellitus, both diagnosed within the last year.

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Home Medications:
- Olanzapine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Von Hippel-Lindau syndrome (father)
- Type 2 diabetes mellitus (mother)
- Bipolar disorder (maternal uncle)
- Hypertension (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
- Blood pressure: 146/92 mmHg

PHYSICAL EXAMINATION:
On physical examination, she has central obesity, a rounded face, a dorsocervical fat pad, and thin skin with purple striae on her abdomen.

LABORATORY DATA:
Initial laboratory tests reveal:

- Fasting glucose: 126 mg/dL
- Hemoglobin A1c: 7.2%

## Encounter 4: source 2021-07-15, practice 2023-11-30, outpatient, Psychiatry

Chief complaint: Routine psychiatric follow-up with weight gain and increased thirst

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Routine psychiatric follow-up with weight gain and increased thirst

HISTORY OF PRESENT ILLNESS:
This 34-year-old female with bipolar I disorder, Cushing syndrome, and diabetes mellitus returns for psychiatric follow-up. She reports feeling more stable on her current medication regimen, which includes olanzapine initiated 6 months ago. However, she has noticed a 10-pound weight gain and increased thirst. She denies any vision changes, polyuria, or genital infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Von Hippel-Lindau syndrome (diagnosed 2020-07-15)
- Cushing syndrome (diagnosed 2021-01-15)
- Bipolar I Disorder
- Type 2 Diabetes Mellitus
- Hypertension
- History of clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Home Medications:
- Olanzapine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for type 2 diabetes mellitus in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, BMI is 28 kg/m^2,

PHYSICAL EXAMINATION:
The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting plasma glucose: 126 mg/dL
- Hemoglobin A1c: 6.8%
- Total cholesterol: 210 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL
- LDL cholesterol (calculated): 130 mg/dL

## Encounter 5: source 2022-01-15, practice 2024-05-27, ed, Emergency Medicine

Chief complaint: Recurrent episodes of confusion and diaphoresis relieved by eating candy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Zhang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of confusion and diaphoresis relieved by eating candy

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with Von Hippel-Lindau syndrome, Cushing syndrome, and diabetes mellitus presents to the emergency department with recurrent episodes of confusion and diaphoresis, primarily occurring in the late afternoon. She reports that these episodes are relieved by eating candy.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Von Hippel-Lindau syndrome (diagnosed 2020-07-15)
- Cushing syndrome (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (diagnosed 2021-07-15)
- Bipolar I Disorder
- Hypertension
- History of clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Her diabetes has been managed with metformin and dietary modifications. She denies using any other medications, including over-the-counter or herbal supplements.

Home Medications:
- Olanzapine 10 mg PO daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Von Hippel-Lindau syndrome (father)
- Type 2 diabetes mellitus (mother)
- Bipolar disorder (maternal uncle)
- Hypertension (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 78 bpm, and she is afebrile.

PHYSICAL EXAMINATION:
On physical examination, she is well-appearing, with a BMI of 29 kg/m^2.

LABORATORY DATA:
A fingerstick glucose measurement during an episode is 45 mg/dL.
The following laboratory values are obtained during an observed hypoglycemic episode:

- Serum glucose: 40 mg/dL
- Insulin level: 45 µU/mL (normal range: 2-25 µU/mL)
- C-peptide level: 1.2 ng/mL (normal range: 1.1-4.4 ng/mL)
- Sulfonylurea screen: Negative

PLAN:
Laboratory tests are ordered, including a sulfonylurea screen, and she is admitted for observation.

## Encounter 6: source 2022-07-15, practice 2024-11-26, outpatient, Neurology

Chief complaint: Headaches, dizziness, and partial vision loss in left eye

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Headaches, dizziness, and partial vision loss in left eye

HISTORY OF PRESENT ILLNESS:
The patient with known Von Hippel-Lindau syndrome returns with recurrent neurological complaints. She reports headaches and episodes of dizziness for the past 6 months and a recent onset of visual disturbances, including blurriness and partial vision loss in her left eye.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Von Hippel-Lindau syndrome (diagnosed 2020-07-15)
- Cushing syndrome (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (diagnosed 2021-07-15)
- Hyperinsulinemia (diagnosed 2022-01-15)
- Bipolar I Disorder
- Hypertension
- History of clear cell renal cell carcinoma

Her medical history is significant for a surgically removed renal mass diagnosed as clear cell renal carcinoma two years ago.

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
Home Medications:
- Olanzapine 10 mg PO daily
- Metformin 1000 mg PO twice daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for a cousin with a similar renal tumor and an uncle who died from complications of a brain tumor.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

VITAL SIGNS:
Blood pressure is 160/95 mmHg

PHYSICAL EXAMINATION:
Fundoscopic examination reveals retinal angiomas

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- Hematocrit: 39.8%
- Fasting glucose: 88 mg/dL
- Serum calcium: 9.6 mg/dL
- Plasma metanephrines: Elevated

IMAGING:
An MRI of the brain shows a cerebellar mass with a nodular enhancement pattern consistent with a hemangioblastoma.

## Encounter 7: source 2024-01-15, practice 2026-05-31, inpatient, Internal Medicine

Chief complaint: Right-sided flank pain, dysuria, fever, and chills for 2 days

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Thompson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Right-sided flank pain, dysuria, fever, and chills for 2 days

HISTORY OF PRESENT ILLNESS:
This 37-year-old female with Von Hippel-Lindau syndrome, a history of clear cell renal cell carcinoma, and Type 2 diabetes mellitus presents with a 2-day history of right-sided flank pain and dysuria. She reports the pain is constant and radiates to the groin. She also notes a fever and chills since yesterday.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus (diagnosed 2020-01-15)
- Polycystic ovary syndrome (diagnosed 2020-04-15)
- Von Hippel-Lindau syndrome (diagnosed 2020-07-15)
- Cushing syndrome (diagnosed 2021-01-15)
- Type 2 diabetes mellitus (diagnosed 2021-07-15)
- Hyperinsulinemia (diagnosed 2022-01-15)
- Bipolar I Disorder
- Hypertension
- History of clear cell renal cell carcinoma

Her medical history is significant for type 2 diabetes mellitus, well-controlled.

PAST SURGICAL HISTORY:
- Left radical nephrectomy for clear cell renal cell carcinoma (age 31)

MEDICATIONS:
She takes on metformin.

Home Medications:
- Olanzapine 10 mg PO daily
- Lisinopril 10 mg PO daily
- Metoprolol succinate 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Von Hippel-Lindau syndrome (father)
- Type 2 diabetes mellitus (mother)
- Bipolar disorder (maternal uncle)
- Hypertension (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: Part-time administrative assistant.

REVIEW OF SYSTEMS:
She denies nausea, vomiting, or diarrhea.

VITAL SIGNS:
On physical examination, her temperature is 38.5°C (101.3°F), blood pressure is 130/85 mmHg, and heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Costovertebral angle tenderness is present on the right side. The patient is alert and oriented, but during the examination, the physician is interrupted multiple times by pages regarding other patients.

LABORATORY DATA:
Urinalysis reveals pyuria and bacteriuria.

Laboratory Values:
- White blood cell count: 12,000 /µL
- Hemoglobin: 13.2 g/dL
- Blood urea nitrogen: 18 mg/dL
- Serum creatinine: 1.0 mg/dL
- Urine culture: Pending
