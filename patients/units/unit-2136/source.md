# synthetic_hospital patient 2136

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental coverage.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-08 (Thu) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2023-06-02 (Fri) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2024-02-06 (Tue) | icu | Cardiology |
| 3 | 2022-03-15 | 2025-02-03 (Mon) | ed | Emergency Medicine |
| 4 | 2023-03-15 | 2026-01-20 (Tue) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim - anaphylaxis",
  "Iodinated contrast - urticaria and bronchospasm (premedication required)"
 ],
 "insurance": "Medicare with supplemental coverage",
 "occupation": "Retired construction supervisor",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Acute myocardial infarction at age 68",
  "Mother: Chronic lymphocytic leukemia, diagnosed age 72",
  "Brother: Hypertension, type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "800 mg PO three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "4000 units subcutaneous weekly",
   "name": "Epoetin alfa"
  },
  {
   "dose": "750 mg/m2 IV every 21 days per R-CHOP protocol",
   "name": "Cyclophosphamide"
  },
  {
   "dose": "375 mg/m2 IV every 21 days per R-CHOP protocol",
   "name": "Rituximab"
  },
  {
   "dose": "300 mg PO daily",
   "name": "Allopurinol"
  },
  {
   "dose": "500 mg PO daily for herpes prophylaxis",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "Subxiphoid pericardial window (performed 8 months ago)",
  "Pericardiocentesis with drain placement (3 prior occasions)",
  "Right subclavian Mediport insertion"
 ],
 "chronic_conditions": [
  "Diffuse large B-cell lymphoma (relapsed) with pericardial involvement",
  "Chronic kidney disease, Stage 3b (baseline eGFR 35-40)",
  "Hypertension",
  "Recurrent malignant pericardial effusion",
  "Anemia of chronic disease"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Recurrent pericardial effusion (palliative management)", "Cardiac tamponade"]
Comorbidities: ["Non-Hodgkin lymphoma", "Malignant pleural effusion"]

## Encounter 0: source 2020-01-15, practice 2022-12-08, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, fatigue, and lightheadedness over the past two weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, fatigue, and lightheadedness over the past two weeks

HISTORY OF PRESENT ILLNESS:
Progressive dyspnea, fatigue, and lightheadedness over the past two weeks. He denies chest pain, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Diffuse large B-cell lymphoma (relapsed) with pericardial involvement
- Chronic kidney disease, Stage 3b (baseline eGFR 35-40)
- Hypertension
- Recurrent malignant pericardial effusion
- Anemia of chronic disease

PAST SURGICAL HISTORY:
- Subxiphoid pericardial window (performed 8 months ago)
- Pericardiocentesis with drain placement (3 prior occasions)
- Right subclavian Mediport insertion

MEDICATIONS:
He takes cyclophosphamide and rituximab.

Home Medications:
- Lisinopril 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily for herpes prophylaxis

ALLERGIES:
Sulfamethoxazole-trimethoprim - anaphylaxis, Iodinated contrast - urticaria and bronchospasm (premedication required)

FAMILY HISTORY:
- Father: Acute myocardial infarction at age 68
- Mother: Chronic lymphocytic leukemia, diagnosed age 72
- Brother: Hypertension, type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths/min, and oxygen saturation is 95% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and heart sounds are distant. There is no lower extremity edema.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
White blood cell count: 7,000/mm^3
Platelet count: 250,000/mm^3
Creatinine: 1.0 mg/dL
BUN: 14 mg/dL
Troponin: <0.01 ng/mL

IMAGING:
Chest X-ray reveals an enlarged cardiac silhouette

OTHER STUDIES:
Electrocardiogram shows low voltage QRS complexes and electrical alternans

## Encounter 1: source 2020-07-15, practice 2023-06-02, ed, Emergency Medicine

Chief complaint: Progressive dyspnea, orthopnea, and chest discomfort relieved by sitting up

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea, orthopnea, and chest discomfort relieved by sitting up

HISTORY OF PRESENT ILLNESS:
The patient, with relapsed diffuse large B-cell lymphoma with pericardial involvement, returns six months after presentation with similar complaints, presenting with progressive dyspnea, orthopnea, and chest discomfort that is relieved by sitting up. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Cardiac tamponade (diagnosed 2020-01-15)
- Diffuse large B-cell lymphoma (relapsed) with pericardial involvement
- Chronic kidney disease, Stage 3b (baseline eGFR 35-40)
- Hypertension
- Recurrent malignant pericardial effusion
- Anemia of chronic disease

PAST SURGICAL HISTORY:
- Subxiphoid pericardial window (performed 8 months ago)
- Pericardiocentesis with drain placement (3 prior occasions)
- Right subclavian Mediport insertion

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly
- Cyclophosphamide 750 mg/m2 IV every 21 days per R-CHOP protocol
- Rituximab 375 mg/m2 IV every 21 days per R-CHOP protocol
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily for herpes prophylaxis

ALLERGIES:
Sulfamethoxazole-trimethoprim - anaphylaxis, Iodinated contrast - urticaria and bronchospasm (premedication required)

FAMILY HISTORY:
- Father: Acute myocardial infarction at age 68
- Mother: Chronic lymphocytic leukemia, diagnosed age 72
- Brother: Hypertension, type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 100/60 mmHg, heart rate is 120 beats per minute, respiratory rate is 24 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated, and heart sounds are distant but no rub is appreciated. Lung fields are clear to auscultation. The patient appears anxious and diaphoretic.

IMAGING:
Echocardiography reveals a large pericardial effusion with evidence of right atrial and ventricular diastolic collapse.

OTHER STUDIES:
An ECG is performed and shows sinus tachycardia with electrical alternans.

## Encounter 2: source 2021-03-15, practice 2024-02-06, icu, Cardiology

Chief complaint: Acute onset of dyspnea, chest pain, and lightheadedness

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Watson, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of dyspnea, chest pain, and lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent pericardial symptoms related to relapsed diffuse large B-cell lymphoma, with prior episodes in January and July 2020. He is status post his last cycle of chemotherapy three weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Cardiac tamponade (diagnosed 2020-01-15)
- Diffuse large B-cell lymphoma (relapsed) with pericardial involvement
- Chronic kidney disease, Stage 3b (baseline eGFR 35-40)
- Hypertension
- Recurrent malignant pericardial effusion
- Anemia of chronic disease

PAST SURGICAL HISTORY:
- Subxiphoid pericardial window (performed 8 months ago)
- Pericardiocentesis with drain placement (3 prior occasions)
- Right subclavian Mediport insertion

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly
- Cyclophosphamide 750 mg/m2 IV every 21 days per R-CHOP protocol
- Rituximab 375 mg/m2 IV every 21 days per R-CHOP protocol
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily for herpes prophylaxis

ALLERGIES:
Sulfamethoxazole-trimethoprim - anaphylaxis, Iodinated contrast - urticaria and bronchospasm (premedication required)

FAMILY HISTORY:
- Father: Acute myocardial infarction at age 68
- Mother: Chronic lymphocytic leukemia, diagnosed age 72
- Brother: Hypertension, type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 85/60 mmHg, heart rate is 110 bpm, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, muffled heart sounds, and clear lungs

IMAGING:
Chest X-ray is unremarkable. Echocardiogram demonstrates a large pericardial effusion with diastolic collapse of the right ventricle.

OTHER STUDIES:
Electrocardiogram shows low voltage QRS complexes and electrical alternans

## Encounter 3: source 2022-03-15, practice 2025-02-03, ed, Emergency Medicine

Chief complaint: Acute onset of dyspnea, lightheadedness, and chest discomfort at rest

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of dyspnea, lightheadedness, and chest discomfort at rest

HISTORY OF PRESENT ILLNESS:
The patient with relapsed diffuse large B-cell lymphoma and recurrent malignant pericardial effusion presents with acute onset dyspnea, lightheadedness, and chest discomfort. He has presented to the emergency department on three prior occasions in 2020 and 2021 for similar symptoms. He reports that these symptoms began suddenly while he was at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Cardiac tamponade (diagnosed 2020-01-15)
- Diffuse large B-cell lymphoma (relapsed) with pericardial involvement
- Chronic kidney disease, Stage 3b (baseline eGFR 35-40)
- Hypertension
- Recurrent malignant pericardial effusion
- Anemia of chronic disease

PAST SURGICAL HISTORY:
- Subxiphoid pericardial window (performed 8 months ago)
- Pericardiocentesis with drain placement (3 prior occasions)
- Right subclavian Mediport insertion

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly
- Cyclophosphamide 750 mg/m2 IV every 21 days per R-CHOP protocol
- Rituximab 375 mg/m2 IV every 21 days per R-CHOP protocol
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily for herpes prophylaxis

ALLERGIES:
Sulfamethoxazole-trimethoprim - anaphylaxis, Iodinated contrast - urticaria and bronchospasm (premedication required)

FAMILY HISTORY:
- Father: Acute myocardial infarction at age 68
- Mother: Chronic lymphocytic leukemia, diagnosed age 72
- Brother: Hypertension, type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 85/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, muffled heart sounds, and clear lungs. The patient's extremities are cool to the touch, and his pulses are thready.

IMAGING:
A bedside echocardiogram demonstrates a large pericardial effusion with diastolic collapse of the right ventricle.

OTHER STUDIES:
An ECG shows low voltage QRS complexes and electrical alternans.

## Encounter 4: source 2023-03-15, practice 2026-01-20, inpatient, Internal Medicine

Chief complaint: Shortness of breath and chest discomfort that began two days ago and progressively worsened

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Shortness of breath and chest discomfort that began two days ago and progressively worsened

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with relapsed diffuse large B-cell lymphoma with pericardial involvement and a history of recurrent cardiac tamponade with four prior episodes from 2020 to 2022 who presents with shortness of breath and chest discomfort that began 2 days ago. He reports that the symptoms have progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Cardiac tamponade (diagnosed 2020-01-15)
- Diffuse large B-cell lymphoma (relapsed) with pericardial involvement
- Chronic kidney disease, Stage 3b (baseline eGFR 35-40)
- Hypertension
- Recurrent malignant pericardial effusion
- Anemia of chronic disease

PAST SURGICAL HISTORY:
The patient underwent a coronary artery bypass graft (CABG) surgery 4 weeks ago.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Epoetin alfa 4000 units subcutaneous weekly
- Cyclophosphamide 750 mg/m2 IV every 21 days per R-CHOP protocol
- Rituximab 375 mg/m2 IV every 21 days per R-CHOP protocol
- Allopurinol 300 mg PO daily
- Valacyclovir 500 mg PO daily for herpes prophylaxis

ALLERGIES:
Sulfamethoxazole-trimethoprim - anaphylaxis, Iodinated contrast - urticaria and bronchospasm (premedication required)

FAMILY HISTORY:
- Father: Acute myocardial infarction at age 68
- Mother: Chronic lymphocytic leukemia, diagnosed age 72
- Brother: Hypertension, type 2 diabetes

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
On examination, his blood pressure is 100/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, distant heart sounds, and no pulmonary crackles. His extremities are cool to touch, and there is no peripheral edema.

IMAGING:
A chest x-ray shows an enlarged cardiac silhouette.

OTHER STUDIES:
An ECG demonstrates low voltage QRS complexes and electrical alternans.
