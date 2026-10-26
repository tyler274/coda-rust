# synthetic_hospital patient 2205

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-21 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-09-13 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2022-12-23 (Fri) | outpatient | Orthopedics |
| 3 | 2023-07-15 | 2024-12-11 (Wed) | outpatient | Hand Surgery |
| 4 | 2024-07-15 | 2025-12-22 (Mon) | outpatient | Hand Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction worker",
 "alcohol_use": "Heavy daily use for decades (approximately 6-8 drinks per day)",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Hypertension",
  "Paternal grandfather: Myocardial infarction at age 72"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
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
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Alcohol use disorder, chronic"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Dupuytren contracture", "Type 2 diabetes mellitus", "Medication noncompliance"]
Comorbidities: ["Alcohol use disorder", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-21, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up.

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms and is adherent to his medications.
The patient inquires about additional treatments to further reduce his cardiovascular risk.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Alcohol use disorder, chronic

MEDICATIONS:
Metformin, lisinopril, and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Paternal grandfather: Myocardial infarction at age 72

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy daily use for decades (approximately 6-8 drinks per day). Occupation: Retired construction worker.

VITAL SIGNS:
His blood pressure is 132/78 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 6.8%
- Fasting plasma glucose: 138 mg/dL
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 90 mg/dL
- HDL cholesterol: 55 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-04-15, practice 2021-09-13, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine follow-up

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with Type 2 diabetes mellitus, hypertension, hyperlipidemia, and alcohol use disorder who returns for routine follow-up three months after his prior visit. He reports adherence to his medication regimen. However, he expresses confusion about the new prescription for his blood pressure that he received from a different provider last week. He recalls the instructions were to take the medication "as directed," but he does not remember the specific dosing instructions.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder, chronic

MEDICATIONS:
Metformin, lisinopril, and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Paternal grandfather: Myocardial infarction at age 72

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy daily use for decades (approximately 6-8 drinks per day). Occupation: Retired construction worker.

REVIEW OF SYSTEMS:
He denies any chest pain, palpitations, or dizziness.

VITAL SIGNS:
His blood pressure today is 158/92 mmHg. His other vital signs are within normal limits

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His most recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- Fasting lipid panel:
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 2: source 2021-07-15, practice 2022-12-23, outpatient, Orthopedics

Chief complaint: comes to the clinic because of progressive difficulty using his right hand.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
He comes to the clinic because of progressive difficulty using his right hand.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with type 2 diabetes mellitus, hypertension, hyperlipidemia, and alcohol use disorder who was previously evaluated in Internal Medicine for routine diabetic follow-up and medication noncompliance, and now presents with progressive difficulty using his right hand. He states that over the past several years he has noticed increasing stiffness and trouble extending some of his fingers, which makes it harder to grasp objects and perform daily tasks. He denies any trauma, redness, or acute pain. He reports no numbness or tingling but states that his hand sometimes feels “tight.” He notes the fingers never “snap back” into place once bent, and the stiffness remains constant rather than intermittent.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Medication noncompliance (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder, chronic

His past medical history includes type 2 diabetes mellitus, hypertension, and chronic alcohol use.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Paternal grandfather: Myocardial infarction at age 72

SOCIAL HISTORY:
He is retired from construction work and spends most of his time at home.

PHYSICAL EXAMINATION:
Examination reveals palpable cords in the palm, and the hand lesion is demonstrated in the image. Sensation and pulses are intact.

## Encounter 3: source 2023-07-15, practice 2024-12-11, outpatient, Hand Surgery

Chief complaint: progressive difficulty using his left hand.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Park, MD
Department: Hand Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty using his left hand.

HISTORY OF PRESENT ILLNESS:
The patient with diabetes returns with progressive left hand difficulty similar to his July 2021 presentation, reporting that over the past several years he has noticed increasing stiffness and inability to fully extend several fingers, which now interferes with daily activities. He denies trauma, infection, or sensory changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Medication noncompliance (diagnosed 2020-04-15)
- Dupuytren contracture (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder, chronic

His medical history includes type 2 diabetes mellitus and hypertension.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Paternal grandfather: Myocardial infarction at age 72

SOCIAL HISTORY:
He is retired and has a long-standing lifestyle habit that places him at greatest risk for this disorder.

PHYSICAL EXAMINATION:
Physical examination shows intact sensation and pulses. The lesion is demonstrated in the image of his hand.

## Encounter 4: source 2024-07-15, practice 2025-12-22, outpatient, Hand Surgery

Chief complaint: comes to the clinic because of progressive difficulty using his right hand

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Hand Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
He comes to the clinic because of progressive difficulty using his right hand

HISTORY OF PRESENT ILLNESS:
The patient returns for progressive right hand symptoms initially evaluated in 2021 with contralateral involvement in 2023. He states that over the past several years he has noticed increasing stiffness and inability to extend some of his fingers, which now interferes with daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Medication noncompliance (diagnosed 2020-04-15)
- Dupuytren contracture (diagnosed 2021-07-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder, chronic

He has type 2 diabetes mellitus.

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Hypertension
- Paternal grandfather: Myocardial infarction at age 72

SOCIAL HISTORY:
He has consumed alcohol heavily for decades. He is retired from construction work.

REVIEW OF SYSTEMS:
He denies recent trauma, redness, or acute pain.

PHYSICAL EXAMINATION:
Examination shows the lesion demonstrated in the image of his hand. Sensation and pulses are intact.

ASSESSMENT AND PLAN:
This disorder is characterized by proliferation of the cell type responsible for
