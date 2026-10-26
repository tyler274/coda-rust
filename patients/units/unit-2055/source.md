# synthetic_hospital patient 2055

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Employer-sponsored private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-21 (Wed) | outpatient | Orthopedics |
| 1 | 2020-07-15 | 2025-02-26 (Wed) | outpatient | Neurology |
| 2 | 2021-07-15 | 2026-02-26 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored private insurance",
 "occupation": "Warehouse freight handler",
 "alcohol_use": "Social, 1-2 beers on weekends",
 "family_history": [
  "Father: Hypertension, obesity",
  "Mother: Chronic lower back pain, degenerative disc disease",
  "Maternal uncle: Cauda equina syndrome requiring emergent surgery"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 8 pack-years, quit 2019",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 10)",
  "No prior spinal surgery"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic lower back pain",
  "Degenerative disc disease",
  "Obesity"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Uncomplicated lumbar radiculopathy", "Cauda equina syndrome"]
Comorbidities: ["Lumbar disc herniation"]

## Encounter 0: source 2020-01-15, practice 2024-08-21, outpatient, Orthopedics

Chief complaint: lower back pain that radiates down the back of his left thigh and calf

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain that radiates down the back of his left thigh and calf

HISTORY OF PRESENT ILLNESS:
He presents to the clinic complaining of lower back pain that radiates down the back of his left thigh and calf, which started after lifting heavy boxes 3 days ago. He describes the pain as a sharp, shooting sensation that is exacerbated by coughing and sitting. He denies any bowel or bladder incontinence.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic lower back pain
- Degenerative disc disease
- Obesity

His past medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 10)
- No prior spinal surgery

MEDICATIONS:
Lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, obesity
- Mother: Chronic lower back pain, degenerative disc disease
- Maternal uncle: Cauda equina syndrome requiring emergent surgery

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2019. Alcohol: Social, 1-2 beers on weekends. Occupation: Warehouse freight handler.

PHYSICAL EXAMINATION:
On physical examination, the patient has a positive straight leg raise test at 40 degrees on the left side. Ankle dorsiflexion strength is 4/5 on the left. Sensation is decreased to pinprick in the left L5 dermatome. Deep tendon reflexes are 2+ and symmetric in the upper extremities and right lower extremity, but the left Achilles reflex is 1+. No saddle anesthesia is noted. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Erythrocyte Sedimentation Rate (ESR): 15 mm/hr (normal)
- C-reactive protein (CRP): 0.8 mg/dL (normal)

## Encounter 1: source 2020-07-15, practice 2025-02-26, outpatient, Neurology

Chief complaint: persistent pain and weakness in his right leg for the past week

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent pain and weakness in his right leg for the past week

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with degenerative disc disease, chronic lower back pain, and obesity who presents with persistent right leg pain and weakness distinct from his prior January 2020 orthopedics encounter involving the left lower extremity. He reports that the pain began after he lifted a heavy toolbox at work, and it initially radiated from his lower back into his right buttock. Over the next few days, the pain extended down the lateral aspect of his right thigh and leg. He now notices difficulty when trying to walk upstairs and says he sometimes drags his right foot when walking on flat ground. He denies bowel or bladder incontinence, and his symptoms have been relatively stable over the past several days.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated lumbar radiculopathy (diagnosed 2020-01-15)
- Hypertension
- Chronic lower back pain
- Degenerative disc disease
- Obesity

PAST SURGICAL HISTORY:
- Tonsillectomy (age 10)
- No prior spinal surgery

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, obesity
- Mother: Chronic lower back pain, degenerative disc disease
- Maternal uncle: Cauda equina syndrome requiring emergent surgery

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2019. Alcohol: Social, 1-2 beers on weekends. Occupation: Warehouse freight handler.

PHYSICAL EXAMINATION:
On neurologic exam, motor strength is 4/5 with dorsiflexion of the right foot. Reflexes are 2+ at the knees and 1+ at the ankles bilaterally. Sensation is decreased over the lateral aspect of the right leg and dorsum of the right foot. Straight-leg raise test reproduces his leg pain at 40 degrees on the right.

## Encounter 2: source 2021-07-15, practice 2026-02-26, ed, Emergency Medicine

Chief complaint: severe lower back pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe lower back pain

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with chronic lower back pain, degenerative disc disease, and lumbar radiculopathy (January and July 2020) who presents to the emergency department with severe lower back pain that has progressively worsened over the past week. The pain radiates to both legs and is associated with numbness around his buttocks and posterior thighs. He reports difficulty initiating urination and a feeling of incomplete bladder emptying.

PAST MEDICAL HISTORY:
Active Problem List:
- Uncomplicated lumbar radiculopathy (diagnosed 2020-01-15)
- Hypertension
- Chronic lower back pain
- Degenerative disc disease
- Obesity

His past medical history is significant for chronic back pain and obesity.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 10)
- No prior spinal surgery

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, obesity
- Mother: Chronic lower back pain, degenerative disc disease
- Maternal uncle: Cauda equina syndrome requiring emergent surgery

SOCIAL HISTORY:
Smoking: Former smoker, 8 pack-years, quit 2019. Alcohol: Social, 1-2 beers on weekends. Occupation: Warehouse freight handler.

PHYSICAL EXAMINATION:
On physical examination, he has decreased sensation in the perianal region and diminished strength in ankle dorsiflexion and plantar flexion bilaterally. The Achilles reflex is absent. Rectal examination reveals decreased anal sphincter tone.

LABORATORY DATA:
Laboratory Values:
- White Blood Cell Count: 7,000/uL (Normal: 4,500-11,000/uL)
- Erythrocyte Sedimentation Rate: 20 mm/hr (Normal: 0-22 mm/hr)
- C-reactive Protein: 5 mg/L (Normal: <10 mg/L)

IMAGING:
An urgent MRI of the lumbar spine shows a large central disc herniation at the L4-L5 level causing significant compression of the cauda equina.
