# synthetic_hospital patient 2287

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 2 | 2021-01-15 | 2026-02-05 (Thu) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2026-08-04 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: Osteoarthritis",
  "Brother: Chronic kidney disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 2018",
 "home_medications": [
  {
   "dose": "30 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale subcutaneous before meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg oral every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus, poorly controlled",
  "Chronic kidney disease, stage 3a",
  "Osteoarthritis, left knee",
  "Hypertension"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Vertebral osteomyelitis", "Septic arthritis"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 2: source 2021-01-15, practice 2026-02-05, ed, Emergency Medicine

Chief complaint: Left knee pain, swelling, and inability to bear weight with low-grade fever

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left knee pain, swelling, and inability to bear weight with low-grade fever

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with poorly controlled diabetes and left knee osteoarthritis presents with a 2-day history of left knee pain, swelling, and inability to bear weight. He denies any recent trauma but reports a low-grade fever and chills at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus, poorly controlled
- Chronic kidney disease, stage 3a
- Osteoarthritis, left knee
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Insulin glargine 30 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous before meals
- Lisinopril 10 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Osteoarthritis
- Brother: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2018. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
The patient's temperature is 38.3°C (100.9°F), heart rate is 102 beats per minute, and other vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, his left knee is erythematous, warm, and markedly swollen with a significantly restricted range of motion due to pain.

LABORATORY DATA:
Aspiration of the knee joint yields synovial fluid with the following analysis:

- Appearance: Cloudy
- WBC count: 50,000 /µL (90% neutrophils)
- Gram stain: No organisms seen
- Glucose: 40 mg/dL
- Protein: 4.5 g/dL

## Encounter 4: source 2021-07-15, practice 2026-08-04, ed, Emergency Medicine

Chief complaint: Progressive lower back pain with fever, chills, and night sweats

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive lower back pain with fever, chills, and night sweats

HISTORY OF PRESENT ILLNESS:
65-year-old male with poorly controlled diabetes, chronic kidney disease, and left knee pain, swelling, and fever six months ago, presents to the emergency department with a 2-week history of progressive lower back pain. He reports the pain is constant, worsens with movement, and is not relieved by over-the-counter analgesics. He also mentions experiencing chills and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic arthritis (diagnosed 2021-01-15)
- Type 2 diabetes mellitus, poorly controlled
- Chronic kidney disease, stage 3a
- Osteoarthritis, left knee
- Hypertension

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Insulin glargine 30 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous before meals
- Lisinopril 10 mg oral daily
- Atorvastatin 40 mg oral daily
- Acetaminophen 500 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: Osteoarthritis
- Brother: Chronic kidney disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 2018. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, he is febrile with a temperature of 38.5°C (101.3°F)

PHYSICAL EXAMINATION:
He has focal tenderness over the L4-L5 vertebrae. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000 /µL (normal: 4,500-11,000 /µL)
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (normal: <10 mg/L)
- Blood cultures: Pending

IMAGING:
Magnetic resonance imaging (MRI) of the lumbar spine reveals an abnormal signal in the L4 and L5 vertebral bodies, with adjacent soft tissue swelling.
