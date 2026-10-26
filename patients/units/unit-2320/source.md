# synthetic_hospital patient 2320

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare Part A & B, Blue Cross Blue Shield supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-27 (Tue) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2023-07-02 (Sun) | inpatient | Internal Medicine |
| 2 | 2021-07-15 | 2024-06-28 (Fri) | inpatient | Otolaryngology |
| 3 | 2022-07-15 | 2025-06-19 (Thu) | inpatient | Otolaryngology |
| 4 | 2023-07-15 | 2026-06-25 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (angioedema)",
  "Penicillin (childhood rash)"
 ],
 "insurance": "Medicare Part A & B, Blue Cross Blue Shield supplemental",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Social (1-2 beers per week)",
 "family_history": [
  "Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction",
  "Mother: Hypertension, stroke at age 74",
  "Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 25 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "42 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale 1:12 ratio with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "600 mg oral three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg oral at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg oral daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "2000 units oral daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Debridement of right plantar foot ulcer (2023)",
  "Cystoscopy with bilateral ureteral stent placement (2021, age 63)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)",
  "Diabetic peripheral neuropathy, severe symmetric distal",
  "Charcot neuroarthropathy, right midfoot",
  "Recurrent complicated urinary tract infections",
  "Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m\u00b2)",
  "Benign prostatic hyperplasia",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Spinal epidural abscess", "Neurogenic arthropathy (Charcot joint)", "Necrotizing otitis externa", "Prerenal acute kidney injury secondary to hypovolemia"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Urinary tract infection", "Stress hyperglycemia"]

## Encounter 0: source 2020-01-15, practice 2022-12-27, outpatient, Endocrinology

Chief complaint: Worsening swelling and pain in right foot with non-healing ulcer near the arch.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening swelling and pain in right foot with non-healing ulcer near the arch.

HISTORY OF PRESENT ILLNESS:
He denies any specific injury but notes that the foot started feeling "puffy" and became more uncomfortable, especially while walking. He also noticed a small ulcer near the arch that has not healed despite daily dressing changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)
- Diabetic peripheral neuropathy, severe symmetric distal
- Charcot neuroarthropathy, right midfoot
- Recurrent complicated urinary tract infections
- Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m²)
- Benign prostatic hyperplasia
- Hypertension
- Hyperlipidemia

He has a history of poorly controlled type 2 diabetes mellitus complicated by peripheral neuropathy.

PAST SURGICAL HISTORY:
- Debridement of right plantar foot ulcer (2023)
- Cystoscopy with bilateral ureteral stent placement (2021, age 63)

MEDICATIONS:
Home Medications:
- Insulin glargine 42 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:12 ratio with meals
- Gabapentin 600 mg oral three times daily
- Lisinopril 10 mg oral daily
- Amlodipine 5 mg oral daily
- Atorvastatin 40 mg oral at bedtime
- Tamsulosin 0.4 mg oral daily
- Aspirin 81 mg oral daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (angioedema), Penicillin (childhood rash)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction
- Mother: Hypertension, stroke at age 74
- Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 8 years ago. Alcohol: Social (1-2 beers per week). Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
On examination, the foot appears warm, erythematous, and mildly deformed at the midfoot, with decreased sensation to light touch and pinprick over the dorsum and sole. There is no tenderness to palpation, and distal pulses are palpable.

IMAGING:
An x-ray of the foot reveals marked disorganization of the tarsometatarsal joints, with bony fragmentation and joint subluxation.

## Encounter 1: source 2020-07-15, practice 2023-07-02, inpatient, Internal Medicine

Chief complaint: Decreased urine output and malaise on day 5 of hospitalization for complicated urinary tract infection.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Torres, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Decreased urine output and malaise on day 5 of hospitalization for complicated urinary tract infection.

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Type 2 Diabetes Mellitus, Charcot neuroarthropathy of the right midfoot diagnosed six months prior, and recurrent complicated urinary tract infections who is currently hospitalized. He was admitted for treatment of a complicated urinary tract infection caused by a Gram-negative bacillus and started on intravenous antibiotics with initial improvement. On day 5 of hospitalization, he develops malaise and decreased urine output.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)
- Diabetic peripheral neuropathy, severe symmetric distal
- Charcot neuroarthropathy, right midfoot
- Recurrent complicated urinary tract infections
- Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m²)
- Benign prostatic hyperplasia
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Debridement of right plantar foot ulcer (2023)
- Cystoscopy with bilateral ureteral stent placement (2021, age 63)

MEDICATIONS:
Home Medications:
- Insulin glargine 42 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:12 ratio with meals
- Gabapentin 600 mg oral three times daily
- Lisinopril 10 mg oral daily
- Amlodipine 5 mg oral daily
- Atorvastatin 40 mg oral at bedtime
- Tamsulosin 0.4 mg oral daily
- Aspirin 81 mg oral daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (angioedema), Penicillin (childhood rash)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction
- Mother: Hypertension, stroke at age 74
- Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 8 years ago. Alcohol: Social (1-2 beers per week). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Vital signs are stable.

LABORATORY DATA:
Laboratory tests reveal serum creatinine of 3.1 mg/dL (baseline 1.0 mg/dL), BUN of 46 mg/dL, and granular muddy-brown casts on urinalysis.

IMAGING:
Renal ultrasound is unremarkable.

PLAN:
The treating team discontinues his current antibiotic regimen.

## Encounter 2: source 2021-07-15, practice 2024-06-28, inpatient, Otolaryngology

Chief complaint: Three-week history of severe constant right ear pain, purulent drainage, and progressive hearing loss.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Voss, MD
Department: Otolaryngology
Visit Type: inpatient

CHIEF COMPLAINT:
Three-week history of severe constant right ear pain, purulent drainage, and progressive hearing loss.

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 Diabetes Mellitus complicated by Charcot neuroarthropathy and prior acute kidney injury presents to the clinic with a 3-week history of right ear pain and drainage. He reports that the pain is severe, constant, and worsens at night. He also complains of difficulty hearing on the affected side.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)
- Diabetic peripheral neuropathy, severe symmetric distal
- Charcot neuroarthropathy, right midfoot
- Recurrent complicated urinary tract infections
- Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m²)
- Benign prostatic hyperplasia
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Debridement of right plantar foot ulcer (2023)
- Cystoscopy with bilateral ureteral stent placement (2021, age 63)

MEDICATIONS:
Home Medications:
- Insulin glargine 42 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:12 ratio with meals
- Gabapentin 600 mg oral three times daily
- Lisinopril 10 mg oral daily
- Amlodipine 5 mg oral daily
- Atorvastatin 40 mg oral at bedtime
- Tamsulosin 0.4 mg oral daily
- Aspirin 81 mg oral daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (angioedema), Penicillin (childhood rash)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction
- Mother: Hypertension, stroke at age 74
- Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 8 years ago. Alcohol: Social (1-2 beers per week). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F).

PHYSICAL EXAMINATION:
On physical examination, there is granulation tissue in the floor of the right external auditory canal, and manipulation of the tragus elicits severe pain. The tympanic membrane is not visible due to edema and discharge.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- Random blood glucose: 250 mg/dL
- HbA1c: 9.2%

IMAGING:
A CT scan of the temporal bones shows bony erosion of the skull base.

## Encounter 3: source 2022-07-15, practice 2025-06-19, inpatient, Otolaryngology

Chief complaint: Recurrent severe right-sided ear pain with purulent otorrhea, difficulty chewing, and new onset hoarse voice.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Voss, MD
Department: Otolaryngology
Visit Type: inpatient

CHIEF COMPLAINT:
Recurrent severe right-sided ear pain with purulent otorrhea, difficulty chewing, and new onset hoarse voice.

HISTORY OF PRESENT ILLNESS:
This patient with poorly controlled diabetes mellitus returns with recurrent severe right-sided otalgia similar to his prior presentation in July 2021. He reports pain worsening at night without relief from over-the-counter analgesics, chewing difficulty secondary to pain, and hoarse voice.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-07-15)
- Necrotizing otitis externa (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)
- Diabetic peripheral neuropathy, severe symmetric distal
- Charcot neuroarthropathy, right midfoot
- Recurrent complicated urinary tract infections
- Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m²)
- Benign prostatic hyperplasia
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Debridement of right plantar foot ulcer (2023)
- Cystoscopy with bilateral ureteral stent placement (2021, age 63)

MEDICATIONS:
Home Medications:
- Insulin glargine 42 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:12 ratio with meals
- Gabapentin 600 mg oral three times daily
- Lisinopril 10 mg oral daily
- Amlodipine 5 mg oral daily
- Atorvastatin 40 mg oral at bedtime
- Tamsulosin 0.4 mg oral daily
- Aspirin 81 mg oral daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (angioedema), Penicillin (childhood rash)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction
- Mother: Hypertension, stroke at age 74
- Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 8 years ago. Alcohol: Social (1-2 beers per week). Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
On examination, the external auditory canal is edematous with visible granulation tissue at the bony-cartilaginous junction. Cranial nerve examination reveals decreased sensation over the right side of the face and weakness of the right facial muscles.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000 /µL
- Erythrocyte sedimentation rate: 85 mm/hr
- Random blood glucose: 250 mg/dL
- HbA1c: 9.2%

IMAGING:
A CT scan of the temporal bones shows bony erosion of the skull base.

## Encounter 4: source 2023-07-15, practice 2026-06-25, ed, Emergency Medicine

Chief complaint: Acute onset severe mid-thoracic back pain with high fever, bilateral leg weakness, and urinary retention.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset severe mid-thoracic back pain with high fever, bilateral leg weakness, and urinary retention.

HISTORY OF PRESENT ILLNESS:
The patient, with poorly controlled diabetes, Charcot neuroarthropathy, and recurrent necrotizing infections, presents with a 3-day history of severe mid-thoracic back pain and fever. He describes the pain as constant, sharp, and exacerbated by movement. He also reports mild leg weakness and difficulty urinating.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic arthropathy (Charcot joint) (diagnosed 2020-01-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-07-15)
- Necrotizing otitis externa (diagnosed 2021-07-15)
- Type 2 Diabetes Mellitus, poorly controlled (HbA1c 9.8%)
- Diabetic peripheral neuropathy, severe symmetric distal
- Charcot neuroarthropathy, right midfoot
- Recurrent complicated urinary tract infections
- Chronic kidney disease, Stage 3b (baseline eGFR 42 mL/min/1.73m²)
- Benign prostatic hyperplasia
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Debridement of right plantar foot ulcer (2023)
- Cystoscopy with bilateral ureteral stent placement (2021, age 63)

MEDICATIONS:
Home Medications:
- Insulin glargine 42 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:12 ratio with meals
- Gabapentin 600 mg oral three times daily
- Lisinopril 10 mg oral daily
- Amlodipine 5 mg oral daily
- Atorvastatin 40 mg oral at bedtime
- Tamsulosin 0.4 mg oral daily
- Aspirin 81 mg oral daily
- Cholecalciferol 2000 units oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (angioedema), Penicillin (childhood rash)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, coronary artery disease, deceased at age 71 from myocardial infarction
- Mother: Hypertension, stroke at age 74
- Older brother: Type 2 diabetes mellitus with end-stage renal disease on hemodialysis

SOCIAL HISTORY:
Smoking: Former smoker, 25 pack-years, quit 8 years ago. Alcohol: Social (1-2 beers per week). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Temperature is 38.7°C (101.7°F), heart rate is 110 bpm, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
There is localized tenderness over the T7-T9 vertebrae. Neurological examination reveals 4/5 strength in the bilateral lower extremities and decreased sensation to light touch below the level of the umbilicus. Reflexes are brisk, and Babinski sign is upgoing bilaterally.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 15,000/mm³ (Normal: 4,500-11,000/mm³)
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 90 mg/L (Normal: <10 mg/L)
- Blood cultures: Pending

IMAGING:
An urgent MRI of the spine is ordered.
