# synthetic_hospital patient 2329

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, African American, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-06 (Mon) | outpatient | Oncology |
| 1 | 2020-07-15 | 2026-03-20 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)",
  "Iodinated contrast (urticaria)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "Occasional, 1-2 beers per week",
 "family_history": [
  "Father: Prostate cancer (diagnosed age 72, deceased)",
  "Paternal uncle: Breast cancer (BRCA2 carrier)",
  "Mother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 15 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "7.5 mg IM every 28 days",
   "name": "Leuprolide depot"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Bicalutamide"
  },
  {
   "dose": "120 mg SC every 4 weeks",
   "name": "Denosumab"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "500 mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "600 mg PO twice daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Transrectal ultrasound-guided prostate biopsy (age 64)",
  "Right hip bone biopsy (age 64, metastatic workup)"
 ],
 "chronic_conditions": [
  "Metastatic prostate adenocarcinoma (diagnosed age 64)",
  "Osteopenia secondary to androgen deprivation therapy",
  "Hypertension"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Cervical myelopathy due to atlantoaxial subluxation", "Prostate Cancer (Suspected)"]
Comorbidities: ["Malignant epidural spinal cord compression", "Prostate Cancer (Suspected)"]

## Encounter 0: source 2020-01-15, practice 2025-10-06, outpatient, Oncology

Chief complaint: Worsening nocturia, urinary hesitancy, and weak urinary stream over two months; unintentional weight loss and intermittent lower back pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening nocturia, urinary hesitancy, and weak urinary stream over two months; unintentional weight loss and intermittent lower back pain

HISTORY OF PRESENT ILLNESS:
A 68-year-old man presents to the clinic with complaints of nocturia, hesitancy, and a weak urinary stream that has progressively worsened over two months. He reports unintentional weight loss and intermittent lower back pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Metastatic prostate adenocarcinoma (diagnosed age 64)
- Osteopenia secondary to androgen deprivation therapy
- Hypertension

PAST SURGICAL HISTORY:
- Transrectal ultrasound-guided prostate biopsy (age 64)
- Right hip bone biopsy (age 64, metastatic workup)

MEDICATIONS:
Home Medications:
- Leuprolide depot 7.5 mg IM every 28 days
- Bicalutamide 50 mg PO daily
- Denosumab 120 mg SC every 4 weeks
- Lisinopril 10 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN pain
- Calcium carbonate 600 mg PO twice daily
- Cholecalciferol 2000 IU PO daily

ALLERGIES:
Sulfa drugs (rash), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father: Prostate cancer (diagnosed age 72, deceased)
- Paternal uncle: Breast cancer (BRCA2 carrier)
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Occasional, 1-2 beers per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On digital rectal examination, the prostate is firm and nodular.

LABORATORY DATA:
His serum prostate-specific antigen is measured at 15.6 ng/mL (reference: < 4.0 ng/mL).

IMAGING:
MRI of the pelvis reveals an irregular prostatic mass with evidence of lytic lesions in the lumbar vertebrae.

PATHOLOGY:
A transrectal ultrasound-guided biopsy confirms adenocarcinoma of the prostate.

PLAN:
The patient undergoes a bilateral orchiectomy as part of his management. Following the procedure,

## Encounter 1: source 2020-07-15, practice 2026-03-20, ed, Emergency Medicine

Chief complaint: Progressive difficulty walking, unsteady gait, and numbness around nipples over three days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive difficulty walking, unsteady gait, and numbness around nipples over three days

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with metastatic prostate adenocarcinoma diagnosed six months ago who presents with progressive neurological symptoms. He currently receives androgen deprivation therapy and denosumab. He recently began experiencing new back pain and unsteady gait. Over the past three days, he has also noticed numbness around his nipples, which he initially attributed to cold weather, but it has persisted indoors. He denies urinary incontinence, but he does report occasional hesitancy and a sense of incomplete bladder emptying. He also describes new weakness in both lower extremities and finds it hard to rise from a chair without using his arms.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate Cancer (Suspected) (diagnosed 2020-01-15)
- Metastatic prostate adenocarcinoma (diagnosed age 64)
- Osteopenia secondary to androgen deprivation therapy
- Hypertension

He was diagnosed with metastatic prostate cancer a year ago.

PAST SURGICAL HISTORY:
- Transrectal ultrasound-guided prostate biopsy (age 64)
- Right hip bone biopsy (age 64, metastatic workup)

MEDICATIONS:
Home Medications:
- Leuprolide depot 7.5 mg IM every 28 days
- Bicalutamide 50 mg PO daily
- Denosumab 120 mg SC every 4 weeks
- Lisinopril 10 mg PO daily
- Acetaminophen 500 mg PO every 6 hours PRN pain
- Calcium carbonate 600 mg PO twice daily
- Cholecalciferol 2000 IU PO daily

ALLERGIES:
Sulfa drugs (rash), Iodinated contrast (urticaria)

FAMILY HISTORY:
- Father: Prostate cancer (diagnosed age 72, deceased)
- Paternal uncle: Breast cancer (BRCA2 carrier)
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Occasional, 1-2 beers per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On examination, his strength is decreased in both legs, and deep tendon reflexes are brisk. Babinski sign is present bilaterally. Sensation to pinprick and temperature is diminished below the level of the nipples, while sensation above this level is intact. Rectal tone is decreased.

IMAGING:
An urgent MRI of the thoracic spine is ordered.

ASSESSMENT AND PLAN:
Involvement of
