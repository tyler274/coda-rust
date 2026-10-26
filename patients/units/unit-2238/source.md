# synthetic_hospital patient 2238

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-13 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-10-13 (Mon) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-04-03 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Multiple myeloma (diagnosed age 71, deceased)",
  "Mother: Hypertension, type 2 diabetes",
  "Brother: Benign prostatic hyperplasia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 2005, 30 pack-year history",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "0.4 mg PO at bedtime",
   "name": "Tamsulosin"
  },
  {
   "dose": "650 mg PO q6h PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "10 mg PO daily on days 1-21 of 28-day cycle",
   "name": "Lenalidomide"
  }
 ],
 "surgical_history": [
  "Bone marrow biopsy (age 68)",
  "Percutaneous nephrostomy tube placement (age 70)"
 ],
 "chronic_conditions": [
  "Multiple myeloma (IgG kappa, diagnosed age 68)",
  "Hypertension",
  "Osteoarthritis",
  "Benign prostatic hyperplasia"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Multiple myeloma cast nephropathy", "Acute kidney injury"]
Comorbidities: ["Essential hypertension", "Dehydration"]

## Encounter 0: source 2020-01-15, practice 2025-04-13, ed, Emergency Medicine

Chief complaint: 2-week history of fatigue, bone pain, and decreased urine output

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fatigue, bone pain, and decreased urine output

HISTORY OF PRESENT ILLNESS:
A 68-year-old male presents to the emergency department with a 2-week history of fatigue, bone pain, and decreased urine output. He reports a 10-pound weight loss over the last month but denies fever, chills, or other systemic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma (IgG kappa, diagnosed age 68)
- Hypertension
- Osteoarthritis
- Benign prostatic hyperplasia

His past medical history is significant for hypertension, for which he takes hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Bone marrow biopsy (age 68)
- Percutaneous nephrostomy tube placement (age 70)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Tamsulosin 0.4 mg PO at bedtime
- Acetaminophen 650 mg PO q6h PRN pain
- Lenalidomide 10 mg PO daily on days 1-21 of 28-day cycle

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Multiple myeloma (diagnosed age 71, deceased)
- Mother: Hypertension, type 2 diabetes
- Brother: Benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 2005, 30 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On physical examination, he appears dehydrated, and his skin turgor is poor.

LABORATORY DATA:
Laboratory studies are ordered:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum chloride: 100 mEq/L
- Serum bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 50 mg/dL
- Serum creatinine: 5.0 mg/dL
- Serum calcium: 12.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Urine sodium: 45 mEq/L
- Urine specific gravity: 1.010

A urine sediment examination shows muddy brown casts.

## Encounter 1: source 2020-07-15, practice 2025-10-13, ed, Emergency Medicine

Chief complaint: 2-day history of nausea, vomiting, and decreased urine output

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of nausea, vomiting, and decreased urine output

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with IgG kappa multiple myeloma and history of cast nephropathy who presents to the emergency department with a 2-day history of nausea, vomiting, and decreased urine output. He reports that he has been unable to keep down any fluids and has not urinated in the last 12 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma cast nephropathy (diagnosed 2020-01-15)
- Multiple myeloma (IgG kappa, diagnosed age 68)
- Hypertension
- Osteoarthritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Bone marrow biopsy (age 68)
- Percutaneous nephrostomy tube placement (age 70)

MEDICATIONS:
His medications include hydrochlorothiazide and tamsulosin.

Home Medications:
- Acetaminophen 650 mg PO q6h PRN pain
- Lenalidomide 10 mg PO daily on days 1-21 of 28-day cycle

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Multiple myeloma (diagnosed age 71, deceased)
- Mother: Hypertension, type 2 diabetes
- Brother: Benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 2005, 30 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired postal worker.

VITAL SIGNS:
On examination, his blood pressure is 95/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
He appears lethargic. His mucous membranes are dry, and his skin turgor is poor.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum creatinine: 2.8 mg/dL (baseline 1.0 mg/dL)
- Blood urea nitrogen (BUN): 70 mg/dL
- Serum potassium: 4.5 mEq/L
- Serum sodium: 135 mEq/L
- Urine specific gravity: 1.030
- Urine sediment: Unremarkable

## Encounter 2: source 2021-01-15, practice 2026-04-03, ed, Emergency Medicine

Chief complaint: Confusion and decreased urine output over the past 2 days

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion and decreased urine output over the past 2 days

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with multiple myeloma and prior acute kidney injury, including cast nephropathy in January 2020 and recurrent AKI in July 2020, presents with confusion and decreased urine output. He recently began taking over-the-counter ibuprofen more frequently for joint pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma cast nephropathy (diagnosed 2020-01-15)
- Acute kidney injury (diagnosed 2020-07-15)
- Multiple myeloma (IgG kappa, diagnosed age 68)
- Hypertension
- Osteoarthritis
- Benign prostatic hyperplasia

His medical history includes hypertension, osteoarthritis, and benign prostatic hyperplasia.

PAST SURGICAL HISTORY:
- Bone marrow biopsy (age 68)
- Percutaneous nephrostomy tube placement (age 70)

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Tamsulosin 0.4 mg PO at bedtime
- Acetaminophen 650 mg PO q6h PRN pain
- Lenalidomide 10 mg PO daily on days 1-21 of 28-day cycle

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Multiple myeloma (diagnosed age 71, deceased)
- Mother: Hypertension, type 2 diabetes
- Brother: Benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 2005, 30 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired postal worker.

REVIEW OF SYSTEMS:
He has not experienced any vomiting, diarrhea, or blood loss.

VITAL SIGNS:
Blood pressure is 122/78 mm Hg, heart rate 92/min.

PHYSICAL EXAMINATION:
On exam, he appears fatigued and has dry mucous membranes. There is no peripheral edema.

LABORATORY DATA:
Laboratory results show:
BUN 58 mg/dL
Creatinine 2.1 mg/dL
FENa                                 0.4%
Urinalysis shows no protein, blood, or casts.

IMAGING:
Bladder scan reveals a post-void residual of 40 mL.
