# synthetic_hospital patient 2134

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO (individual marketplace plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-17 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2026-03-26 (Thu) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO (individual marketplace plan)",
 "occupation": "Self-employed carpenter and part-time delivery driver",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Essential hypertension",
  "Brother: Bilateral carpal tunnel syndrome"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 2018)",
 "home_medications": [
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "400 mg orally every 6 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2015)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Obesity (BMI 32)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Ulnar neuropathy at the elbow (Cubital tunnel syndrome)"]
Comorbidities: ["Occupational mechanical strain"]

## Encounter 0: source 2020-01-15, practice 2025-03-17, outpatient, Internal Medicine

Chief complaint: Progressive numbness and weakness in right hand for the past month

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and weakness in right hand for the past month

HISTORY OF PRESENT ILLNESS:
He works as a delivery driver and spends several hours daily gripping a steering wheel. He reports tingling and decreased sensation in his right pinky and the adjacent side of the ring finger. Over the past week, he's had difficulty opening jars and performing fine motor tasks like buttoning his shirt. He denies neck pain, trauma, or any symptoms in the opposite limb.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Obesity (BMI 32)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Ibuprofen 400 mg orally every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Essential hypertension
- Brother: Bilateral carpal tunnel syndrome

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018). Alcohol: Occasional (1-2 drinks per week). Occupation: Self-employed carpenter and part-time delivery driver.

PHYSICAL EXAMINATION:
On exam, there is mild atrophy of the hypothenar eminence and decreased sensation over the medial aspect of the hand. Strength testing reveals impaired finger abduction and adduction. The patient is asked to hold a piece of paper between the thumb and index finger; he compensates by flexing the thumb at the interphalangeal joint. The rest of the neurologic exam is normal.

## Encounter 1: source 2021-01-15, practice 2026-03-26, outpatient, Neurology

Chief complaint: Worsening progressive weakness in right hand affecting work performance

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening progressive weakness in right hand affecting work performance

HISTORY OF PRESENT ILLNESS:
The patient with hypertension and obesity returns for follow-up evaluation of right hand weakness initially assessed by Internal Medicine one year ago. He presents with worsening symptoms affecting work performance and says he has trouble gripping tools and typing.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulnar neuropathy at the elbow (Cubital tunnel syndrome) (diagnosed 2020-01-15)
- Hypertension
- Obesity (BMI 32)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2015)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Ibuprofen 400 mg orally every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Essential hypertension
- Brother: Bilateral carpal tunnel syndrome

SOCIAL HISTORY:
He works as a carpenter

PHYSICAL EXAMINATION:
On examination, there is wasting of the hypothenar eminence and interosseous muscles. When asked to extend all his fingers, his 4th and 5th digits remain flexed at the interphalangeal joints and hyperextended at the metacarpophalangeal joints. Sensation is decreased over the medial side of the hand and the fifth digit. Wrist flexion is preserved. Tinel sign is positive at the medial elbow.
