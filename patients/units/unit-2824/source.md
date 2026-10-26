# synthetic_hospital patient 2824

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-07 (Thu) | ed | General Surgery |
| 1 | 2020-07-15 | 2026-01-29 (Thu) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-07-29 (Wed) | ed | Orthopedic Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 67",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago",
 "home_medications": [
  {
   "dose": "5 mg PO daily",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg PO BID",
   "name": "Metoprolol tartrate"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Open cholecystectomy",
  "Inguinal hernia repair"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Atrial Fibrillation"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Coagulopathy due to anticoagulant overdose", "Left femoral neck fracture with warfarin-induced coagulopathy", "Postoperative hemorrhage following inguinal hernia repair"]
Comorbidities: ["Chronic atrial fibrillation", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-08-07, ed, General Surgery

Chief complaint: Increasing confusion and agitation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion and agitation

HISTORY OF PRESENT ILLNESS:
She is brought to the emergency department by her family due to increasing confusion and agitation that began 12 hours ago. She underwent an open cholecystectomy 5 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation

Her past medical history includes hypertension, type 2 diabetes mellitus, and chronic atrial fibrillation for which she takes.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Inguinal hernia repair

MEDICATIONS:
She takes metoprolol, metformin, and warfarin respectively.

Home Medications:
- Metoprolol tartrate 50 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, she is afebrile, blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Her abdomen is distended with diffuse tenderness to palpation, but no rebound tenderness or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Her hemoglobin has dropped from 12 g/dL preoperatively to 8 g/dL. Her INR is 3.5.

IMAGING:
Abdominal ultrasound shows free fluid in the peritoneal cavity.

## Encounter 1: source 2020-07-15, practice 2026-01-29, ed, Emergency Medicine

Chief complaint: Sudden onset of severe lower back pain that started 8 hours ago

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe lower back pain that started 8 hours ago

HISTORY OF PRESENT ILLNESS:
This 78-year-old female with chronic atrial fibrillation on warfarin, previously hospitalized for postoperative hemorrhage, presents with sudden severe lower back pain of eight hours duration. She denies any trauma or heavy lifting. Her pain is non-radiating and does not improve with changes in position. She also reports feeling lightheaded and has had episodes of shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation

Her past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Inguinal hernia repair

MEDICATIONS:
She takes warfarin therapy.

Home Medications:
- Metoprolol tartrate 50 mg PO BID
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
Her back is tender to palpation, but there is no external sign of injury or bruising. Her abdomen is soft and non-distended, but she winces when the flank areas are palpated.

LABORATORY DATA:
Hemoglobin: 9.2 g/dL (baseline 13.5 g/dL)
- Hematocrit: 27.8%
- INR: 2.1
- Creatinine: 1.0 mg/dL
- Platelet count: 250,000/mm^3

IMAGING:
An urgent abdominal CT scan without contrast shows a large retroperitoneal hematoma.

## Encounter 2: source 2021-01-15, practice 2026-07-29, ed, Orthopedic Surgery

Chief complaint: Severe pain in her left hip

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Orthopedic Surgery
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in her left hip

HISTORY OF PRESENT ILLNESS:
This 79-year-old female with diabetes, atrial fibrillation on warfarin, and bleeding complications is brought to the emergency department after falling at home. She has not been able to bear weight on it since the fall.

PAST MEDICAL HISTORY:
Active Problem List:
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2020-01-15)
- Coagulopathy due to anticoagulant overdose (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Atrial Fibrillation

Her past medical history includes hypertension, type 2 diabetes mellitus, and chronic atrial fibrillation.

PAST SURGICAL HISTORY:
- Open cholecystectomy
- Inguinal hernia repair

MEDICATIONS:
She takes metoprolol, metformin, and warfarin.

Home Medications:
- Metoprolol tartrate 50 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 67
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, her left leg is shortened and externally rotated.

LABORATORY DATA:
Initial laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 250 x 10^9/L
- INR: 3.8
- Creatinine: 1.1 mg/dL
- Blood glucose: 180 mg/dL

IMAGING:
A hip X-ray confirms a left femoral neck fracture.

OTHER STUDIES:
Her ECG shows atrial fibrillation with a controlled ventricular rate.
