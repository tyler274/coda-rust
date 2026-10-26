# synthetic_hospital patient 2853

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex F, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-19 (Mon) | outpatient | Orthopedic Surgery |
| 1 | 2020-07-15 | 2021-04-14 (Wed) | outpatient | Vascular Surgery |
| 2 | 2022-01-15 | 2022-10-06 (Thu) | outpatient | Cardiology |
| 3 | 2023-07-15 | 2024-04-04 (Thu) | outpatient | Cardiology |
| 4 | 2025-01-15 | 2025-10-19 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Retired retail manager",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Hypertension, hemorrhagic stroke at age 74",
  "Father: Coronary artery disease, hypertension",
  "Sister: Varicose veins, chronic venous insufficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 25 pack-year history, quit 18 months ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "500 mg PO PRN for knee pain (max 3g daily)",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Endovascular coiling of ruptured cerebral aneurysm",
  "Total abdominal hysterectomy (age 52)",
  "Laparoscopic cholecystectomy (age 45)"
 ],
 "chronic_conditions": [
  "Obesity (BMI 33)",
  "Hypertension (diagnosed 12 years ago, previously poorly controlled)",
  "Hypertensive retinopathy",
  "Varicose veins with venous stasis changes",
  "Osteoarthritis, bilateral knees",
  "Heart failure with preserved ejection fraction (HFpEF)"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Chronic venous insufficiency", "Osteoarthritis of knee", "Ruptured cerebral aneurysm", "Heart failure with preserved ejection fraction"]
Comorbidities: ["Essential hypertension", "Left ventricular hypertrophy", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2020-10-19, outpatient, Orthopedic Surgery

Chief complaint: Bilateral knee pain with progressive worsening over three years, worse with walking and prolonged standing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain with progressive worsening over three years, worse with walking and prolonged standing

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with progressively worsening bilateral knee pain that began over three years ago. She describes the pain as dull and aching, most noticeable after walking or prolonged standing. She denies morning stiffness, systemic symptoms, or joint redness.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (BMI 33)
- Hypertension (diagnosed 12 years ago, previously poorly controlled)
- Hypertensive retinopathy
- Varicose veins with venous stasis changes
- Osteoarthritis, bilateral knees
- Heart failure with preserved ejection fraction (HFpEF)

Her past medical history includes obesity and venous stasis.

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured cerebral aneurysm
- Total abdominal hysterectomy (age 52)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN for knee pain (max 3g daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, hemorrhagic stroke at age 74
- Father: Coronary artery disease, hypertension
- Sister: Varicose veins, chronic venous insufficiency

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-year history, quit 18 months ago. Alcohol: None. Occupation: Retired retail manager.

VITAL SIGNS:
Her BMI is 42 kg/m².

PHYSICAL EXAMINATION:
On exam, she has crepitus with passive knee motion and visible bony enlargement of the distal interphalangeal joints. No erythema or warmth is present.

LABORATORY DATA:
Synovial fluid aspirated from the right knee is clear with a WBC count of 950/μL and normal glucose levels.

IMAGING:
A plain radiograph shows asymmetric joint space narrowing and evidence of subchondral sclerosis.

## Encounter 1: source 2020-07-15, practice 2021-04-14, outpatient, Vascular Surgery

Chief complaint: Progressive lower leg swelling and heaviness worsening by end of day, improves with elevation

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive lower leg swelling and heaviness worsening by end of day, improves with elevation

HISTORY OF PRESENT ILLNESS:
She returns with lower extremity complaints and known varicose veins with venous stasis changes and prior knee evaluation. She reports a 6-month history of progressive swelling, discomfort, and sensation of heaviness in her lower legs that worsens by the end of the day and improves with leg elevation.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Obesity (BMI 33)
- Hypertension (diagnosed 12 years ago, previously poorly controlled)
- Hypertensive retinopathy
- Varicose veins with venous stasis changes
- Osteoarthritis, bilateral knees
- Heart failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured cerebral aneurysm
- Total abdominal hysterectomy (age 52)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN for knee pain (max 3g daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, hemorrhagic stroke at age 74
- Father: Coronary artery disease, hypertension
- Sister: Varicose veins, chronic venous insufficiency

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-year history, quit 18 months ago. Alcohol: None. Occupation: Retired retail manager.

PHYSICAL EXAMINATION:
Bilateral lower extremity pitting edema, erythema, and scaling around the medial malleoli. There are also areas of superficial erosion and a few red-brown patches of discoloration.

OTHER STUDIES:
Her ankle brachial index is normal.

## Encounter 2: source 2022-01-15, practice 2022-10-06, outpatient, Cardiology

Chief complaint: Increasing fatigue, mild shortness of breath, and occasional nocturnal dyspnea with declining exercise tolerance

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, mild shortness of breath, and occasional nocturnal dyspnea with declining exercise tolerance

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of heart failure with preserved ejection fraction and hypertension, with prior orthopedic and vascular evaluations. Over the past several months, she denies chest pain or palpitations but reports occasional nocturnal dyspnea. Her exercise tolerance has declined over the past year, and she can no longer walk more than a few blocks without becoming winded.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Chronic venous insufficiency (diagnosed 2020-07-15)
- Obesity (BMI 33)
- Hypertension (diagnosed 12 years ago, previously poorly controlled)
- Hypertensive retinopathy
- Varicose veins with venous stasis changes
- Osteoarthritis, bilateral knees
- Heart failure with preserved ejection fraction (HFpEF)

She has a history of long-standing hypertension.

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured cerebral aneurysm
- Total abdominal hysterectomy (age 52)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She takes amlodipine and hydrochlorothiazide.

Home Medications:
- Acetaminophen 500 mg PO PRN for knee pain (max 3g daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, hemorrhagic stroke at age 74
- Father: Coronary artery disease, hypertension
- Sister: Varicose veins, chronic venous insufficiency

SOCIAL HISTORY:
She does not smoke or drink alcohol

VITAL SIGNS:
Her blood pressure is 148/88 mm Hg and heart rate is 74/min

PHYSICAL EXAMINATION:
She has jugular venous distension with prominent a waves, bibasilar crackles on lung auscultation, and bilateral pitting edema in the lower extremities. Cardiac auscultation reveals a normal S1 and S2 with an additional low-pitched S4 sound at the apex. There are no murmurs, rubs, or gallops. Her lungs are clear to percussion and there is no hepatomegaly.

LABORATORY DATA:
BNP is mildly elevated. Her renal and hepatic panels are within normal limits.

IMAGING:
An echocardiogram shows preserved contractile function with concentric thickening of the left ventricle. Chest x-ray shows mild pulmonary vascular congestion but no cardiomegaly.

OTHER STUDIES:
A 12-lead ECG shows normal sinus rhythm with evidence of left ventricular hypertrophy

## Encounter 3: source 2023-07-15, practice 2024-04-04, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion with dizziness on standing, symptoms more pronounced over past year

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion with dizziness on standing, symptoms more pronounced over past year

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of dyspnea initially evaluated in January 2022, with ongoing knee pain and leg swelling. She reports that climbing stairs now leaves her winded, and she has also noticed occasional dizziness after standing up too quickly, though she denies syncope or chest pain. Her symptoms have become more pronounced in the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Chronic venous insufficiency (diagnosed 2020-07-15)
- Heart failure with preserved ejection fraction (diagnosed 2022-01-15)
- Obesity (BMI 33)
- Hypertension (diagnosed 12 years ago, previously poorly controlled)
- Hypertensive retinopathy
- Varicose veins with venous stasis changes
- Osteoarthritis, bilateral knees
- Heart failure with preserved ejection fraction (HFpEF)

She has a history of long-standing hypertension. She has not seen an eye doctor in years but recalls being told previously about some vascular changes in her retina.

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured cerebral aneurysm
- Total abdominal hysterectomy (age 52)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
She is currently taking amlodipine.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN for knee pain (max 3g daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of early cardiac disease

SOCIAL HISTORY:
She does not smoke or drink alcohol

VITAL SIGNS:
Blood pressure of 148/88 mm Hg and a heart rate of 72/min

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a sustained apical impulse with a low-pitched extra heart sound immediately preceding S1. Her lungs are clear, and there is no peripheral edema.

IMAGING:
A transthoracic echocardiogram is performed and shown below.

## Encounter 4: source 2025-01-15, practice 2025-10-19, ed, Emergency Medicine

Chief complaint: Sudden thunderclap headache described as worst of life, witnessed collapse with brief loss of consciousness

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden thunderclap headache described as worst of life, witnessed collapse with brief loss of consciousness

HISTORY OF PRESENT ILLNESS:
This 69-year-old female with hypertension, HFpEF, and osteoarthritis followed in Cardiology, Orthopedics, and Vascular Surgery since 2020 is brought to the ED after developing a sudden, excruciating headache described as the "worst of her life" while walking up a flight of stairs. She screamed, collapsed to the ground, and briefly lost consciousness. She reports severe generalized headache with neck stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Chronic venous insufficiency (diagnosed 2020-07-15)
- Heart failure with preserved ejection fraction (diagnosed 2022-01-15)
- Obesity (BMI 33)
- Hypertension (diagnosed 12 years ago, previously poorly controlled)
- Hypertensive retinopathy
- Varicose veins with venous stasis changes
- Osteoarthritis, bilateral knees
- Heart failure with preserved ejection fraction (HFpEF)

PAST SURGICAL HISTORY:
- Endovascular coiling of ruptured cerebral aneurysm
- Total abdominal hysterectomy (age 52)
- Laparoscopic cholecystectomy (age 45)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Acetaminophen 500 mg PO PRN for knee pain (max 3g daily)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypertension, hemorrhagic stroke at age 74
- Father: Coronary artery disease, hypertension
- Sister: Varicose veins, chronic venous insufficiency

SOCIAL HISTORY:
Tobacco use

VITAL SIGNS:
Her vital signs include BP 184/102 mm Hg and HR 88/min.

PHYSICAL EXAMINATION:
On arrival, she is drowsy and photophobic. Neurologic exam is non-focal.

IMAGING:
A non-contrast CT head is inconclusive.

OTHER STUDIES:
A lumbar puncture reveals elevated RBCs and a yellowish color of the CSF.
