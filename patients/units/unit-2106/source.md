# synthetic_hospital patient 2106

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-19 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-07-15 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-01-21 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Disabled former construction worker",
 "alcohol_use": "None",
 "family_history": [
  "Type 2 diabetes mellitus (mother)",
  "Coronary artery disease (father)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 5 years ago",
 "home_medications": [
  {
   "dose": "50/200/25 mg PO daily",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "400 mg PO every 6 hours as needed for pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "500 mg PO every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Thoracic epidural steroid injection (2 months prior)",
  "L2 vertebroplasty (3 months prior)"
 ],
 "chronic_conditions": [
  "HIV infection with AIDS",
  "Diabetes mellitus type 2, poorly controlled",
  "Chronic lower back pain",
  "Osteoporosis",
  "History of L2 vertebral compression fracture"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Spinal epidural abscess", "Acute bacterial meningitis"]
Comorbidities: ["Immunosuppressed state"]

## Encounter 0: source 2020-01-15, practice 2025-01-19, ed, Emergency Medicine

Chief complaint: worsening mid-thoracic back pain and fever with difficulty initiating urination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening mid-thoracic back pain and fever with difficulty initiating urination

HISTORY OF PRESENT ILLNESS:
2-week history of worsening back pain and fever. He reports the pain is localized to the mid-thoracic region and has become increasingly severe, not relieved by his usual regimen of ibuprofen and acetaminophen. He also notes tingling in his legs and difficulty initiating urination. He had an epidural steroid injection for his chronic back pain 3 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection with AIDS
- Diabetes mellitus type 2, poorly controlled
- Chronic lower back pain
- Osteoporosis
- History of L2 vertebral compression fracture

PAST SURGICAL HISTORY:
- Thoracic epidural steroid injection (2 months prior)
- L2 vertebroplasty (3 months prior)

MEDICATIONS:
He takes ibuprofen and acetaminophen.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/200/25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Coronary artery disease (father)

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Disabled former construction worker.

VITAL SIGNS:
Temperature is 38.7°C, heart rate is 110 bpm, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Neurological examination reveals decreased sensation to light touch below the level of T6 and 4/5 strength in the lower extremities bilaterally. Rectal examination shows decreased anal sphincter tone.

LABORATORY DATA:
- White blood cell count: 15,000 /µL
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- C-reactive protein (CRP): 90 mg/L

IMAGING:
An urgent MRI of the spine is ordered

## Encounter 1: source 2020-07-15, practice 2025-07-15, ed, Emergency Medicine

Chief complaint: severe lower back pain progressively worsening over 1 week with subjective fevers

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe lower back pain progressively worsening over 1 week with subjective fevers

HISTORY OF PRESENT ILLNESS:
This 55-year-old male with HIV/AIDS and prior spinal epidural abscess returns with similar complaints. He denies any recent invasive procedures or skin infections. The pain is described as a constant, dull ache that is not relieved by over-the-counter analgesics and is exacerbated by movement. He also reports subjective fevers and chills at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal epidural abscess (diagnosed 2020-01-15)
- HIV infection with AIDS
- Diabetes mellitus type 2, poorly controlled
- Chronic lower back pain
- Osteoporosis
- History of L2 vertebral compression fracture

PAST SURGICAL HISTORY:
- Thoracic epidural steroid injection (2 months prior)
- L2 vertebroplasty (3 months prior)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/200/25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Ibuprofen 400 mg PO every 6 hours as needed for pain
- Acetaminophen 500 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Coronary artery disease (father)

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Disabled former construction worker.

VITAL SIGNS:
Temperature of 38.7°C (101.7°F), heart rate of 110 bpm, and blood pressure of 135/85 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is febrile. Neurological examination reveals mild weakness in the bilateral lower extremities and decreased sensation to light touch over the L3 to S1 dermatomes.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 15,000 /µL (normal: 4,500-11,000 /µL)
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (normal: 0-22 mm/hr)
- C-reactive protein (CRP): 90 mg/L (normal: <5.0 mg/L)
- Blood cultures: No growth to date

IMAGING:
Magnetic resonance imaging (MRI) of the lumbar spine shows an epidural mass at the L2 level with bone involvement of the adjacent vertebrae.

## Encounter 2: source 2021-01-15, practice 2026-01-21, ed, Emergency Medicine

Chief complaint: acute onset fever, severe headache, and neck stiffness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset fever, severe headache, and neck stiffness

HISTORY OF PRESENT ILLNESS:
This patient with HIV infection with AIDS and prior spinal epidural abscesses presents with a 2-day history of fever, headache, and neck stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Spinal epidural abscess (diagnosed 2020-01-15)
- HIV infection with AIDS
- Diabetes mellitus type 2, poorly controlled
- Chronic lower back pain
- Osteoporosis
- History of L2 vertebral compression fracture

PAST SURGICAL HISTORY:
- Thoracic epidural steroid injection (2 months prior)
- L2 vertebroplasty (3 months prior)

MEDICATIONS:
He takes on antiretroviral therapy.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/200/25 mg PO daily
- Metformin 1000 mg PO twice daily
- Insulin glargine 20 units subcutaneous at bedtime
- Ibuprofen 400 mg PO every 6 hours as needed for pain
- Acetaminophen 500 mg PO every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Coronary artery disease (father)

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago. Alcohol: None. Occupation: Disabled former construction worker.

VITAL SIGNS:
He has a temperature of 38.9°C, heart rate of 110 bpm, and blood pressure of 100/60 mmHg

PHYSICAL EXAMINATION:
Physical examination reveals nuchal rigidity and a positive Brudzinski's sign. He is alert but appears lethargic.

LABORATORY DATA:
His CD4 count is 180 cells/mm³

PLAN:
A non-contrast CT scan of the head is ordered to rule out any contraindications to lumbar puncture. While awaiting the CT results,
