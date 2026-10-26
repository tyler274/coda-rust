# synthetic_hospital patient 2334

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare Part B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-14 (Thu) | outpatient | Otolaryngology |
| 1 | 2020-07-15 | 2026-02-23 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part B",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Father: Myocardial infarction at age 76",
  "Mother: Alzheimer's disease",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg orally as needed for joint pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 65)",
  "Cataract extraction, bilateral (age 68)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Osteoarthritis"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Zenker diverticulum"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-08-14, outpatient, Otolaryngology

Chief complaint: Several months of progressive difficulty swallowing with coughing fits during meals

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of progressive difficulty swallowing with coughing fits during meals

HISTORY OF PRESENT ILLNESS:
He describes the sensation of food sticking in his throat shortly after swallowing, particularly with solids. He also reports coughing fits during meals and a recent tendency to avoid eating in public due to embarrassment. Occasionally, he notes the spontaneous regurgitation of food hours after eating. His wife has noticed a foul smell on his breath, even after brushing his teeth. The patient has also experienced episodes of waking up with choking and a sensation of fluid in his mouth. He denies chest pain, heartburn, unintentional weight loss, night sweats, or recent travel. His appetite is normal, and his bowel movements have not changed. He has tried chewing more thoroughly and eating slowly, but symptoms persist.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
- Cholecystectomy (age 65)
- Cataract extraction, bilateral (age 68)

MEDICATIONS:
He takes a daily multivitamin and low-dose aspirin for primary prevention.

Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 20 mg orally daily
- Acetaminophen 500 mg orally as needed for joint pain

ALLERGIES:
The patient has no known food allergies.

FAMILY HISTORY:
- Father: Myocardial infarction at age 76
- Mother: Alzheimer's disease
- Brother: Hypertension

SOCIAL HISTORY:
He does not smoke and drinks alcohol only socially.

VITAL SIGNS:
On physical examination, vital signs are within normal limits: temperature is 36.9°C (98.4°F), blood pressure is 124/78 mm Hg, pulse is 76/min, and respiratory rate is 14/min

PHYSICAL EXAMINATION:
Oral examination is unremarkable. There are no cervical lymph nodes or masses. Cardiopulmonary and abdominal examinations are normal. Neurologic exam is nonfocal.

IMAGING:
A barium swallow is done and shown below.

## Encounter 1: source 2020-07-15, practice 2026-02-23, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing solids and liquids with nocturnal choking

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Okafor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solids and liquids with nocturnal choking

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with hypertension, hyperlipidemia, and osteoarthritis returns following his January 2020 Otolaryngology visit. He reports a 1-year history of progressive difficulty swallowing solids and liquids, frequent regurgitation of undigested food, and halitosis. His wife notes he often coughs and sometimes wakes at night choking.

PAST MEDICAL HISTORY:
Active Problem List:
- Zenker diverticulum (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
- Cholecystectomy (age 65)
- Cataract extraction, bilateral (age 68)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg orally daily
- Multivitamin 1 tablet daily
- Lisinopril 10 mg orally daily
- Atorvastatin 20 mg orally daily
- Acetaminophen 500 mg orally as needed for joint pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 76
- Mother: Alzheimer's disease
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Retired high school mathematics teacher.

PHYSICAL EXAMINATION:
On exam, he has halitosis, gurgling in the neck with palpation, and no cervical lymphadenopathy.

IMAGING:
A barium swallow demonstrates a posterior outpouching in the hypopharyngeal region.
