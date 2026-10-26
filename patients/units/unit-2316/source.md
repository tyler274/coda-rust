# synthetic_hospital patient 2316

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-13 (Wed) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2025-11-07 (Fri) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-05-07 (Thu) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Ischemic stroke at age 72",
  "Brother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "25 mg PO BID",
   "name": "Carvedilol"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Spironolactone"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (LAD) 3 years ago",
  "Implantable cardioverter-defibrillator (ICD) placement 2 years ago"
 ],
 "chronic_conditions": [
  "Heart failure with reduced ejection fraction (HFrEF), EF 30-35%",
  "Coronary artery disease",
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Uncomplicated parapneumonic effusion", "Right middle cerebral artery stroke"]
Comorbidities: ["Heart failure with reduced ejection fraction", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2025-08-13, outpatient, Cardiology

Chief complaint: whether it's safe for him to drink sports drinks like Gatorade

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Whether it's safe for him to drink sports drinks like Gatorade

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (HFrEF), EF 30-35%
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD) 3 years ago
- Implantable cardioverter-defibrillator (ICD) placement 2 years ago

MEDICATIONS:
Home Medications:
- Carvedilol 25 mg PO BID
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 25 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

## Encounter 1: source 2020-04-15, practice 2025-11-07, ed, Emergency Medicine

Chief complaint: increasing shortness of breath and right-sided chest discomfort for the past 3 days

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath and right-sided chest discomfort for the past 3 days

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with heart failure with reduced ejection fraction, coronary artery disease, and diabetes mellitus who was last evaluated in January 2020 regarding counseling on sports drink consumption. He presents to the emergency department with increasing shortness of breath and right-sided chest discomfort for the past 3 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Heart failure with reduced ejection fraction (HFrEF), EF 30-35%
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

He has a history of heart failure with reduced ejection fraction.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD) 3 years ago
- Implantable cardioverter-defibrillator (ICD) placement 2 years ago

MEDICATIONS:
Home Medications:
- Carvedilol 25 mg PO BID
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 25 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On examination, there is decreased breath sounds and dullness to percussion over the right lower lung field beginning at the level of the 5th rib and extending downward. He is afebrile and in no acute distress.

IMAGING:
A chest x-ray confirms a large right pleural effusion.

OTHER STUDIES:
The patient is seated upright and leaning forward over a bedside table in preparation for a diagnostic thoracentesis. After skin preparation and local anesthesia, at

## Encounter 2: source 2020-10-15, practice 2026-05-07, icu, Neurology

Chief complaint: confusion and difficulty attending to his surroundings

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Maria Rodriguez, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Confusion and difficulty attending to his surroundings

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with HFrEF (EF 30-35%), CAD, HTN, T2DM, and HLD, most recently evaluated six months prior for an uncomplicated parapneumonic effusion, is brought to the emergency department by his wife due to confusion and difficulty attending to his surroundings. She reports that since waking up this morning, he has ignored food on the left side of his plate and does not respond when spoken to from his left side. He shaves only the right half of his face and seems unaware of the problem.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated parapneumonic effusion (diagnosed 2020-04-15)
- Heart failure with reduced ejection fraction (HFrEF), EF 30-35%
- Coronary artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (LAD) 3 years ago
- Implantable cardioverter-defibrillator (ICD) placement 2 years ago

MEDICATIONS:
Home Medications:
- Carvedilol 25 mg PO BID
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Spironolactone 25 mg PO daily
- Aspirin 81 mg PO daily
- Atorvastatin 40 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Brother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On neurologic exam, he is alert and oriented, with normal strength and intact language function. When asked to draw a clock, he places all the numbers on the right side of the circle.

IMAGING:
MRI reveals an acute infarct in the right cerebral hemisphere.
