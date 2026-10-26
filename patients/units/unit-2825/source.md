# synthetic_hospital patient 2825

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 73 at first encounter, sex F, White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-24 (Fri) | outpatient | Neurology |
| 1 | 2020-07-15 | 2022-06-28 (Tue) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-01-15 | 2022-12-19 (Mon) | outpatient | Gastroenterology |
| 3 | 2022-01-15 | 2023-12-29 (Fri) | outpatient | Neurology |
| 4 | 2023-01-15 | 2024-12-29 (Sun) | icu | Critical Care |
| 5 | 2023-07-15 | 2025-06-23 (Mon) | inpatient | Internal Medicine |
| 6 | 2024-01-15 | 2025-12-25 (Thu) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2026-06-13 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Mother with type 2 diabetes mellitus",
  "Father with coronary artery disease",
  "Sister with Alzheimer's disease"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Former smoker, quit 20 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "24 units SC at bedtime",
   "name": "Insulin Glargine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "10 mg PO TID before meals",
   "name": "Metoclopramide"
  }
 ],
 "surgical_history": [
  "No prior major surgeries"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Obstructive Pulmonary Disease",
  "Chronic Kidney Disease Stage 3",
  "Hyperlipidemia",
  "Osteoporosis",
  "Coronary Artery Disease",
  "Alzheimer's Disease",
  "Gastroparesis"
 ],
 "age_at_first_encounter": 73
}
```

Primary diagnoses: ["Severe Symptomatic Pelvic Organ Prolapse", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Acute kidney injury", "Alzheimer's disease", "Displaced fracture of left femoral neck", "Hyperosmolar hyperglycemic state", "Pancreatic cystic neoplasm with high-risk features", "Drug-induced parkinsonism due to metoclopramide"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-12-24, outpatient, Neurology

Chief complaint: Memory concerns and progressive cognitive decline noted by family over the past two years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Memory concerns and progressive cognitive decline noted by family over the past two years.

HISTORY OF PRESENT ILLNESS:
Over the past two years, the patient has had progressive difficulty remembering recent events, such as appointments and conversations. The daughter reports that her mother has become increasingly withdrawn and occasionally agitated, particularly in the evening. Recently, the patient has started claiming that people are stealing from her, although there is no evidence to support this.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Alzheimer's Disease
- Gastroparesis

Her past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Her medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin Glargine 24 units SC at bedtime
- Hydrochlorothiazide 25 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears apathetic. She is oriented to person but not to time or place. Her Mini-Mental State Examination score is 18 out of 30. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory studies show:
- Serum glucose: 140 mg/dL
- Serum sodium: 142 mEq/L
- Serum creatinine: 1.0 mg/dL
- Serum calcium: 9.2 mg/dL
- TSH: 2.5 mIU/L

## Encounter 1: source 2020-07-15, practice 2022-06-28, outpatient, Obstetrics & Gynecology

Chief complaint: Sensation of pelvic heaviness and vaginal bulge with urinary difficulty.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Sensation of pelvic heaviness and vaginal bulge with urinary difficulty.

HISTORY OF PRESENT ILLNESS:
This 73-year-old female with Alzheimer’s disease, evaluated by Neurology in January, presents to Gynecology with sensation of pelvic heaviness and vaginal bulge with urinary difficulty that has progressively worsened over the past year. She reports difficulty with urination including a weak stream and a feeling of incomplete bladder emptying, and denies fecal incontinence or rectal prolapse.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

Her medical history is significant for hypertension, type 2 diabetes mellitus, and chronic obstructive pulmonary disease (COPD).

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Her medications include metformin, lisinopril, and tiotropium.

Home Medications:
- Insulin Glargine 24 units SC at bedtime
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
The patient is not sexually active

VITAL SIGNS:
Blood pressure: 145/90 mmHg
Body mass index (BMI): 32 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, a protrusion of the cervix and vaginal walls is noted beyond the vaginal introitus with Valsalva maneuver. The cervix appears atrophic, and there is no evidence of ulceration.

LABORATORY DATA:
Hemoglobin A1c: 7.8%

OTHER STUDIES:
Her most recent Papanicolaou test was normal.

PLAN:
Expresses a strong preference for avoiding surgery due to her fear of anesthesia and her multiple comorbidities.

## Encounter 2: source 2021-01-15, practice 2022-12-19, outpatient, Gastroenterology

Chief complaint: Intermittent upper abdominal pain radiating to the back for three months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent upper abdominal pain radiating to the back for three months.

HISTORY OF PRESENT ILLNESS:
The patient, who was diagnosed with Alzheimer’s disease in January 2020 and evaluated for pelvic organ prolapse in July 2020, presents to the clinic with a 3-month history of intermittent abdominal pain, primarily located in the upper abdomen. She describes the pain as dull and radiating to her back. She denies any recent changes in weight, appetite, or bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

She has a history of type 2 diabetes mellitus and hypertension, which are well-controlled.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
She takes with metformin and lisinopril, respectively.

Home Medications:
- Insulin Glargine 24 units SC at bedtime
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for breast cancer in her mother.

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft and non-distended, with mild tenderness in the epigastric region. No jaundice or palpable masses are noted.

LABORATORY DATA:
Laboratory tests reveal normal liver function tests and a serum amylase within normal limits.

IMAGING:
An abdominal ultrasound shows a 4 cm cystic lesion in the head of the pancreas with a thickened, irregular wall and calcifications. There is no evidence of biliary ductal dilatation. The main pancreatic duct is not visualized on ultrasound.

ASSESSMENT AND PLAN:
Given the patient's presentation and ultrasound findings,

## Encounter 3: source 2022-01-15, practice 2023-12-29, outpatient, Neurology

Chief complaint: Gradually worsening tremor, rigidity, and bradykinesia over the past month.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening tremor, rigidity, and bradykinesia over the past month.

HISTORY OF PRESENT ILLNESS:
This 75-year-old female with Alzheimer's disease and gastroparesis returns to Neurology for evaluation of progressive motor symptoms, following prior evaluation in this department in 2020 and recent Gastroenterology follow-up. She reports a resting tremor in her hands, stiffness in her limbs, and a general sense of slowness that has made it increasingly difficult to perform daily activities like cooking and buttoning clothes. Her daughter notes that she has developed a "blank stare," has stopped smiling as much, and walks with reduced arm swing. The patient denies falls, cognitive changes, hallucinations, or mood symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Pancreatic cystic neoplasm with high-risk features (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

Her medical history includes type 2 diabetes mellitus complicated by gastroparesis, hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Current medications include insulin glargine, lisinopril, simvastatin, low-dose aspirin, and metoclopramide.

Home Medications:
- Metformin 1000 mg PO BID
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Tiotropium 18 mcg inhaled daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no personal or family history of neurodegenerative disease.

SOCIAL HISTORY:
She does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
On examination, she appears expressionless with reduced blinking and a mildly stooped posture. She has cogwheel rigidity in both upper extremities, decreased arm swing bilaterally, and a 4-Hz resting tremor in her right hand. Gait is slow and shuffling. Muscle strength and deep tendon reflexes are normal, and sensation is intact. Mental status is unremarkable.

## Encounter 4: source 2023-01-15, practice 2024-12-29, icu, Critical Care

Chief complaint: Acute confusion, lethargy, polydipsia, and polyuria.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Thompson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute confusion, lethargy, polydipsia, and polyuria.

HISTORY OF PRESENT ILLNESS:
The patient is a 76-year-old female with Type 2 Diabetes Mellitus, Alzheimer's disease diagnosed in 2020, and prior metoclopramide-induced parkinsonism identified in 2022, who is brought to the emergency department by her daughter due to confusion and lethargy. Over the past 2 days, the patient has become increasingly somnolent and disoriented. The daughter mentions that her mother has been drinking water constantly and urinating every hour. She has no history of head trauma, recent illness, or alcohol use.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Pancreatic cystic neoplasm with high-risk features (diagnosed 2021-01-15)
- Drug-induced parkinsonism due to metoclopramide (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

She has a history of poorly controlled type 2 diabetes mellitus, hypertension, and coronary artery disease.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Her medications include insulin, hydrochlorothiazide, lisinopril, and atorvastatin.

Home Medications:
- Metformin 1000 mg PO BID
- Insulin Glargine 24 units SC at bedtime
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her temperature is 36.5°C (97.7°F), blood pressure is 105/68 mm Hg, and pulse is 112/min.

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place, her mucous membranes are dry, and her skin turgor is poor.

LABORATORY DATA:
Laboratory studies show: Sodium 127 mEq/L Potassium 3.7 mEq/L Chloride 94 mEq/L Bicarbonate 22 mEq/L Blood urea nitrogen 26 mg/dL Glucose 790 mg/dL Creatinine 1.6 mg/dL

## Encounter 5: source 2023-07-15, practice 2025-06-23, inpatient, Internal Medicine

Chief complaint: Community-acquired pneumonia with superimposed acute kidney injury.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Community-acquired pneumonia with superimposed acute kidney injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Pancreatic cystic neoplasm with high-risk features (diagnosed 2021-01-15)
- Drug-induced parkinsonism due to metoclopramide (diagnosed 2022-01-15)
- Hyperosmolar hyperglycemic state (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
She reports taking multiple medications at home, including metformin, lisinopril, and a "water pill." Her son, who accompanies her, provides a list of her medications, but is unsure about the dosages.

Home Medications:
- Insulin Glargine 24 units SC at bedtime
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On admission, her blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals crackles in the right lower lung field.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 2.1 mg/dL (baseline 1.5 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL
- Serum potassium: 5.2 mEq/L
- Hemoglobin A1c: 7.8%

IMAGING:
Chest X-ray shows a right lower lobe infiltrate.

PLAN:
The patient is started on appropriate antibiotics for pneumonia.

## Encounter 6: source 2024-01-15, practice 2025-12-25, ed, Emergency Medicine

Chief complaint: Fall at home with severe right hip pain and inability to bear weight.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Zhang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fall at home with severe right hip pain and inability to bear weight.

HISTORY OF PRESENT ILLNESS:
This 77-year-old female with Alzheimer's disease, osteoporosis, and prior drug-induced parkinsonism presents following a mechanical fall. She reports slipping on a wet floor and landing on her right side. She complains of severe pain in her right hip and is unable to bear weight on the right leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Pancreatic cystic neoplasm with high-risk features (diagnosed 2021-01-15)
- Drug-induced parkinsonism due to metoclopramide (diagnosed 2022-01-15)
- Hyperosmolar hyperglycemic state (diagnosed 2023-01-15)
- Acute kidney injury (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

Her past medical history includes osteoporosis, hypertension, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Insulin Glargine 24 units SC at bedtime
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, her right leg appears shortened and is externally rotated. There is tenderness to palpation over the right hip, and movement of the hip joint exacerbates the pain. The neurovascular examination of the right leg is intact.

LABORATORY DATA:
Laboratory values:
- Calcium: 9.2 mg/dL (normal: 8.6-10.2 mg/dL)
- Hemoglobin A1c: 7.8% (normal: <5.7% for non-diabetics)
- Vitamin D: 20 ng/mL (normal: 30-100 ng/mL)

IMAGING:
A radiograph of the right hip shows a displaced fracture through the femoral neck.

## Encounter 7: source 2024-07-15, practice 2026-06-13, ed, Emergency Medicine

Chief complaint: Acute shortness of breath and fatigue without chest pain.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute shortness of breath and fatigue without chest pain.

HISTORY OF PRESENT ILLNESS:
The patient with coronary artery disease, chronic obstructive pulmonary disease, and recent hip fracture presents with acute symptoms. She reports that she has been feeling "under the weather" for the past week but attributed it to her chronic obstructive pulmonary disease (COPD). She denies chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (diagnosed 2020-01-15)
- Severe Symptomatic Pelvic Organ Prolapse (diagnosed 2020-07-15)
- Pancreatic cystic neoplasm with high-risk features (diagnosed 2021-01-15)
- Drug-induced parkinsonism due to metoclopramide (diagnosed 2022-01-15)
- Hyperosmolar hyperglycemic state (diagnosed 2023-01-15)
- Acute kidney injury (diagnosed 2023-07-15)
- Displaced fracture of left femoral neck (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Obstructive Pulmonary Disease
- Chronic Kidney Disease Stage 3
- Hyperlipidemia
- Osteoporosis
- Coronary Artery Disease
- Gastroparesis

Her past medical history includes hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Her medications include lisinopril, simvastatin, and metformin.

Home Medications:
- Insulin Glargine 24 units SC at bedtime
- Hydrochlorothiazide 25 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Tiotropium 18 mcg inhaled daily
- Metoclopramide 10 mg PO TID before meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Father with coronary artery disease
- Sister with Alzheimer's disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: No alcohol use. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals mild bilateral wheezing. Cardiac examination shows a regular rhythm without murmurs, rubs, or gallops.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows ST-segment elevations in leads II, III, and aVF.
