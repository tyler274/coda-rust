# synthetic_hospital patient 2027

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Medicare (primary), Blue Cross Blue Shield (secondary).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-11-04 (Fri) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2023-05-08 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-04-15 | 2024-01-24 (Wed) | outpatient | General Surgery |
| 3 | 2021-10-15 | 2024-07-31 (Wed) | outpatient | Neurology |
| 4 | 2022-04-15 | 2025-02-07 (Fri) | outpatient | Neurology |
| 5 | 2022-10-15 | 2025-07-25 (Fri) | outpatient | Neurology |
| 6 | 2023-04-15 | 2026-01-26 (Mon) | outpatient | Neurology |
| 7 | 2023-10-15 | 2026-08-10 (Mon) | outpatient | Nephrology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (primary), Blue Cross Blue Shield (secondary)",
 "occupation": "Project Manager (currently on medical leave)",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Father: Hypertension, Myocardial Infarction at age 67",
  "Mother: Type 2 Diabetes Mellitus, Stroke at age 72",
  "One brother: Hypertension",
  "No family history of neurodegenerative diseases or carcinoid syndrome"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "50 mg PO BID",
   "name": "Riluzole"
  },
  {
   "dose": "30 mg IM monthly",
   "name": "Octreotide LAR"
  }
 ],
 "surgical_history": [
  "Open appendectomy (2 years prior)",
  "Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)"
 ],
 "chronic_conditions": [
  "Essential Hypertension (long-standing, variable control)",
  "Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis",
  "Left Ventricular Hypertrophy",
  "Amyotrophic Lateral Sclerosis",
  "Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)",
  "Incisional Hernia (post-operative)"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Carcinoid syndrome (enterochromaffin cell neoplasm)", "Incisional hernia", "Left ventricular hypertrophy", "Hyperkalemia", "Amyotrophic lateral sclerosis"]
Comorbidities: ["Essential hypertension", "Status post right hemicolectomy"]

## Encounter 0: source 2020-01-15, practice 2022-11-04, outpatient, Cardiology

Chief complaint: Worsening shortness of breath on exertion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Robert Martinez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening shortness of breath on exertion

PAST MEDICAL HISTORY:
Active Problem List:
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Left Ventricular Hypertrophy
- Amyotrophic Lateral Sclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

He has a history of long-standing hypertension that is poorly controlled.

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

VITAL SIGNS:
Blood pressure of 168/96 mmHg

PHYSICAL EXAMINATION:
Laterally displaced point of maximal impulse. Cardiac auscultation reveals normal S1 and S2 with no murmurs

LABORATORY DATA:
Laboratory evaluation is normal

IMAGING:
Echocardiography shows increased thickness of the left ventricular wall with reduced compliance and normal chamber size. There is no evidence of valvular disease

## Encounter 1: source 2020-07-15, practice 2023-05-08, outpatient, Gastroenterology

Chief complaint: 3-month history of episodic nausea, abdominal cramping, and watery diarrhea

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of episodic nausea, abdominal cramping, and watery diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with carcinoid syndrome on octreotide returns for follow-up, having last been seen in Cardiology six months ago. He reports that these symptoms often occur after meals and are sometimes accompanied by facial warmth and flushing. He has unintentionally lost 10 pounds over the last 2 months. He denies recent travel, antibiotic use, or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Amyotrophic Lateral Sclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

PAST SURGICAL HISTORY:
Remote appendectomy

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, he appears well-nourished but has mild hepatomegaly. His stool is guaiac-negative, and no rash is noted.

LABORATORY DATA:
Laboratory evaluation reveals mild niacin deficiency. A 24-hour urine test shows elevated 5-hydroxyindoleacetic acid (5-HIAA) levels.

IMAGING:
A CT scan of abdomen demonstrates 4 cm ileal mass.

PATHOLOGY:
The mass is resected and is shown below with both microscopic and gross views.

## Encounter 2: source 2021-04-15, practice 2024-01-24, outpatient, General Surgery

Chief complaint: 6-month history of a slowly enlarging mass in the lower abdomen

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Jennifer Walsh, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of a slowly enlarging mass in the lower abdomen

HISTORY OF PRESENT ILLNESS:
This 59-year-old with amyotrophic lateral sclerosis and carcinoid syndrome (per prior Gastroenterology and Cardiology encounters) presents with six-month enlarging lower abdominal mass. He reports that the mass becomes more prominent when he coughs or strains.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Amyotrophic Lateral Sclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

His past medical history is significant for an open appendectomy 2 years ago.

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, a soft, non-tender bulge is noted in the right lower quadrant of the abdomen, which enlarges with the Valsalva maneuver and reduces when the patient lies supine. There is no overlying skin erythema or warmth.

IMAGING:
Abdominal ultrasound shows a defect in the anterior abdominal wall with protrusion of omental fat.

## Encounter 3: source 2021-10-15, practice 2024-07-31, outpatient, Neurology

Chief complaint: 6-month history of progressive weakness in his hands and difficulty walking

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. James Patterson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of progressive weakness in his hands and difficulty walking

HISTORY OF PRESENT ILLNESS:
The patient with hypertension, CKD stage 3, LVH, carcinoid syndrome, and incisional hernia presents for neurologic evaluation. He reports that his grip strength has diminished, making it hard to hold objects, and he frequently trips over his own feet. He also mentions occasional muscle cramps and twitches in his arms and legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Incisional hernia (diagnosed 2021-04-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Amyotrophic Lateral Sclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, you note atrophy of the intrinsic hand muscles and fasciculations in the upper extremities. His reflexes are brisk in the lower extremities, and he has a positive Babinski sign on the right. Sensation is intact, and there is no evidence of bowel or bladder dysfunction.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Serum electrolytes: within normal limits
- Thyroid function tests: within normal limits
- Creatine kinase: slightly elevated

OTHER STUDIES:
Electrodiagnostic studies show findings consistent with both acute and chronic denervation.

## Encounter 4: source 2022-04-15, practice 2025-02-07, outpatient, Neurology

Chief complaint: 6-month history of progressive weakness in his upper and lower extremities

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. James Patterson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of progressive weakness in his upper and lower extremities

HISTORY OF PRESENT ILLNESS:
He returns for follow-up, reporting difficulty buttoning his shirt and climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Incisional hernia (diagnosed 2021-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-10-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On examination, you note atrophy of the hand muscles, fasciculations in the arms, and hyperreflexia in the lower extremities. His sensory examination is normal.

OTHER STUDIES:
Spirometry reveals a reduced vital capacity (VC) and forced expiratory volume in 1 second (FEV1), with a normal FEV1/forced vital capacity (FVC) ratio. Maximal inspiratory and expiratory pressures are decreased. The diffusion capacity for carbon monoxide (DLCO) is within normal limits.

Spirometry Results:
- Vital Capacity (VC): Decreased
- Forced Expiratory Volume in 1 second (FEV1): Decreased
- Forced Vital Capacity (FVC): Decreased
- FEV1/FVC ratio: Normal
- Maximal Inspiratory Pressure: Decreased
- Maximal Expiratory Pressure: Decreased
- Diffusion Capacity for Carbon Monoxide (DLCO): Normal

## Encounter 5: source 2022-10-15, practice 2025-07-25, outpatient, Neurology

Chief complaint: Progressive difficulty in swallowing and slurred speech

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. James Patterson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty in swallowing and slurred speech

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of amyotrophic lateral sclerosis, last evaluated 6 months ago for progressive weakness. He reports a 6-month history of progressive difficulty in swallowing and slurred speech. He also reports occasional choking while drinking liquids. His wife has noticed he has become more clumsy and has stumbled several times while walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Incisional hernia (diagnosed 2021-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-10-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On examination, you observe atrophy and fasciculations of the tongue. His speech is nasal and spastic. The jaw jerk reflex is brisk, and there is increased tone in the upper extremities with hyperreflexia. His gait is spastic. There is no sensory deficit, and cognitive function is intact.

LABORATORY DATA:
Routine blood work, including complete blood count, electrolytes, renal function, and thyroid function tests, is within normal limits.

## Encounter 6: source 2023-04-15, practice 2026-01-26, outpatient, Neurology

Chief complaint: Progressive difficulty in climbing stairs and an inability to grip objects tightly

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. James Patterson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty in climbing stairs and an inability to grip objects tightly

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with essential hypertension, CKD stage 3, LVH, metastatic neuroendocrine tumor, and incisional hernia, returning for ALS follow-up. He presents to the clinic with a 6-month history of progressive difficulty in climbing stairs and an inability to grip objects tightly, as well as occasional slurring of speech. His wife has noticed he seems to be breathing shallowly, especially at night, and has bouts of uncontrolled laughing and crying.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Incisional hernia (diagnosed 2021-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-10-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

His past medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

PHYSICAL EXAMINATION:
On physical examination, you note normal sensory function, but observe atrophy of the hand muscles and fasciculations in the upper extremities. His reflexes are brisk, and plantar responses are extensor.

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.42
- PaCO2: 45 mmHg
- PaO2: 70 mmHg
- HCO3-: 24 mEq/L

OTHER STUDIES:
Pulmonary function tests reveal a forced vital capacity (FVC) of 60% predicted.

## Encounter 7: source 2023-10-15, practice 2026-08-10, outpatient, Nephrology

Chief complaint: Presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2023-10-15
Provider: Dr. Lisa Thompson, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
This 61-year-old with chronic kidney disease stage 3 returns for follow-up. He reports compliance with his medications, including lisinopril and amlodipine. He has been on a low-potassium diet as advised during his last visit.

PAST MEDICAL HISTORY:
Active Problem List:
- Left ventricular hypertrophy (diagnosed 2020-01-15)
- Carcinoid syndrome (enterochromaffin cell neoplasm) (diagnosed 2020-07-15)
- Incisional hernia (diagnosed 2021-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-10-15)
- Essential Hypertension (long-standing, variable control)
- Chronic Kidney Disease Stage 3 secondary to hypertensive nephrosclerosis
- Metastatic Neuroendocrine Tumor (Carcinoid Syndrome)
- Incisional Hernia (post-operative)

PAST SURGICAL HISTORY:
- Open appendectomy (2 years prior)
- Exploratory laparotomy with small bowel resection (for primary neuroendocrine tumor, 18 months prior)

MEDICATIONS:
He takes lisinopril and amlodipine.

Home Medications:
- Riluzole 50 mg PO BID
- Octreotide LAR 30 mg IM monthly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, Myocardial Infarction at age 67
- Mother: Type 2 Diabetes Mellitus, Stroke at age 72
- One brother: Hypertension
- No family history of neurodegenerative diseases or carcinoid syndrome

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: Rare (1-2 drinks per month). Occupation: Project Manager (currently on medical leave).

REVIEW OF SYSTEMS:
He denies any symptoms of muscle weakness or palpitations

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Serum potassium: 5.6 mEq/L (Normal range: 3.5-5.0 mEq/L)
- Serum creatinine: 2.1 mg/dL (Normal range: 0.6-1.2 mg/dL)
- Estimated GFR: 35 mL/min/1.73m^2
- Serum bicarbonate: 24 mEq/L (Normal range: 22-29 mEq/L)
- Serum calcium: 9.2 mg/dL (Normal range: 8.5-10.2 mg/dL)
