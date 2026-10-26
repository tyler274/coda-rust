# synthetic_hospital patient 2698

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex F, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-09 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-11-28 (Fri) | inpatient | Internal Medicine |
| 2 | 2021-01-15 | 2026-06-12 (Fri) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Regional sales manager",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Father with deep vein thrombosis (age 62)",
  "Maternal grandmother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (2010)",
  "Right carpal tunnel release (2015)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Rheumatoid arthritis"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["Iliofemoral Deep Vein Thrombosis", "Acute deep vein thrombosis of left popliteal vein", "Adrenal crisis"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2025-06-09, ed, Emergency Medicine

Chief complaint: Progressive left calf pain and swelling for 3 days following recent long car trip

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive left calf pain and swelling for 3 days following recent long car trip

HISTORY OF PRESENT ILLNESS:
She reports no recent trauma but mentions a long car trip 2 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Rheumatoid arthritis

Her medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (2010)
- Right carpal tunnel release (2015)

MEDICATIONS:
She takes metformin and lisinopril.

Home Medications:
- Prednisone 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with deep vein thrombosis (age 62)
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, her left calf is erythematous, warm, and tender to palpation. There is pitting edema extending from the ankle to the mid-calf. The circumference of the left calf is 3 cm greater than the right. The rest of the examination is unremarkable.

LABORATORY DATA:
Her renal function is normal

IMAGING:
A Doppler ultrasound of the lower extremities reveals a noncompressible left popliteal vein consistent with deep vein thrombosis (DVT).

## Encounter 1: source 2020-07-15, practice 2025-11-28, inpatient, Internal Medicine

Chief complaint: Progressive left calf and thigh pain and swelling for two days following transatlantic flight

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive left calf and thigh pain and swelling for two days following transatlantic flight

HISTORY OF PRESENT ILLNESS:
The patient with type 2 diabetes, hypertension, rheumatoid arthritis on prednisone, and prior left popliteal DVT presents to the emergency department with a two-day history of progressive left calf pain and swelling. She reports no recent trauma but mentions that she returned from a transatlantic flight three days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute deep vein thrombosis of left popliteal vein (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Rheumatoid arthritis

She has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (2010)
- Right carpal tunnel release (2015)

MEDICATIONS:
Her medications include metformin and lisinopril.

Home Medications:
- Prednisone 5 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with deep vein thrombosis (age 62)
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Regional sales manager.

VITAL SIGNS:
Her vital signs are stable, and the rest of the examination is unremarkable.

PHYSICAL EXAMINATION:
On physical examination, her left calf is erythematous, warm, and tender to palpation. There is pitting edema extending from the left calf to the ankle. The circumference of the left calf is 3 cm greater than the right. Homan's sign is positive.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count: Within normal limits
- Serum Creatinine: 1.0 mg/dL
- D-dimer: Elevated

## Encounter 2: source 2021-01-15, practice 2026-06-12, icu, Internal Medicine

Chief complaint: Sudden onset of severe abdominal pain and vomiting after slip and fall at home

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and vomiting after slip and fall at home

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old female with type 2 diabetes mellitus, hypertension, rheumatoid arthritis on chronic corticosteroid therapy, and a history of recurrent lower extremity deep vein thrombosis who presents to the emergency department with severe abdominal pain and vomiting. She reports that her symptoms started suddenly a few hours ago after she slipped and fell at home. She denies any head injury or loss of consciousness. Initial fluid resuscitation with normal saline has been initiated, but her blood pressure remains low.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute deep vein thrombosis of left popliteal vein (diagnosed 2020-01-15)
- Iliofemoral Deep Vein Thrombosis (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Rheumatoid arthritis

Her medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (2010)
- Right carpal tunnel release (2015)

MEDICATIONS:
Chronic prednisone therapy, lisinopril, metformin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with deep vein thrombosis (age 62)
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: Regional sales manager.

VITAL SIGNS:
On examination, her blood pressure is 85/50 mmHg, heart rate is 110 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
Her abdomen is tender to palpation, but there is no rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.1 mEq/L)
- Blood urea nitrogen (BUN): 25 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
- Glucose: 92 mg/dL (Normal: 70-99 mg/dL)
- White blood cell count: 12,000/mm³ (Normal: 4,500-11,000/mm³)
