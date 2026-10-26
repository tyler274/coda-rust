# synthetic_hospital patient 2024

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-08 (Thu) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-07-12 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2022-04-15 (Fri) | outpatient | Internal Medicine |
| 3 | 2022-01-15 | 2023-04-07 (Fri) | outpatient | Nephrology |
| 4 | 2023-01-15 | 2024-04-22 (Mon) | outpatient | Podiatry |
| 5 | 2023-07-15 | 2024-10-14 (Mon) | follow_up | Podiatry |
| 6 | 2024-07-15 | 2025-10-06 (Mon) | follow_up | General Surgery |
| 7 | 2025-01-15 | 2026-04-07 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Construction laborer",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, hypertension",
  "Paternal grandfather: Colorectal cancer (diagnosed age 65)",
  "Older brother: Paget disease of bone",
  "Mother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "per protocol",
   "name": "Investigational agent (clinical trial for diabetic nephropathy)"
  }
 ],
 "surgical_history": [
  "Primary suturing of left forearm deep laceration",
  "Hemicolectomy with primary anastomosis for colonic mass",
  "Multiple bedside debridements of left lower extremity ulcers"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (15-year duration)",
  "Hypertension",
  "Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy",
  "Peripheral Arterial Disease",
  "Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)",
  "Colorectal adenocarcinoma",
  "Paget disease of bone"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Diabetic nephropathy", "Diabetic motor neuropathy", "Colorectal neoplasia", "Diabetic foot ulcer in proliferative phase of healing", "Paget disease of bone", "Rhinocerebral mucormycosis", "Wound healing, early maturation phase (3 weeks post-injury)"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus", "Diabetic nephropathy"]

## Encounter 0: source 2020-01-15, practice 2021-04-08, outpatient, Endocrinology

Chief complaint: Routine health maintenance visit for chronic Paget disease of bone with mild fatigue.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit for chronic Paget disease of bone with mild fatigue.

HISTORY OF PRESENT ILLNESS:
He reports mild fatigue but no bone pain, fractures, or recent illnesses.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma
- Paget disease of bone

His history is notable for type II diabetes mellitus.

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory tests reveal normal serum creatinine and liver function tests. The following nomogram represents his serum alkaline phosphatase (ALP) and calcium concentrations.

## Encounter 1: source 2020-04-15, practice 2021-07-12, outpatient, Endocrinology

Chief complaint: Six-month history of burning pain in feet that worsens at night with occasional electric shock sensations and difficulty sensing feet when walking.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Wilson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of burning pain in feet that worsens at night with occasional electric shock sensations and difficulty sensing feet when walking.

HISTORY OF PRESENT ILLNESS:
He returns following January Paget evaluation. He also reports that he has difficulty sensing his feet when walking, leading to frequent tripping.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
His diabetes has been managed with metformin and dietary modifications.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, the patient has diminished sensation to light touch and pinprick in a stocking distribution. Vibration sense is reduced at the ankles, and proprioception is impaired. Ankle reflexes are absent bilaterally.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Fasting blood glucose: 156 mg/dL
- HbA1c: 8.2%
- Serum creatinine: 1.1 mg/dL
- Urinary albumin-to-creatinine ratio: 45 mg/g

## Encounter 2: source 2021-01-15, practice 2022-04-15, outpatient, Internal Medicine

Chief complaint: Two-month history of intermittent fever, night sweats, and ten-pound unintentional weight loss without bowel habit changes or bleeding.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of intermittent fever, night sweats, and ten-pound unintentional weight loss without bowel habit changes or bleeding.

HISTORY OF PRESENT ILLNESS:
This 59-year-old male with diabetes and colorectal adenocarcinoma, followed in Endocrinology, returns. He reports 2-month history of intermittent fever, night sweats, and 10-pound unintentional weight loss. He denies bowel habit changes, rectal bleeding, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

VITAL SIGNS:
Temperature is 38.2°C, blood pressure is 135/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Cardiac examination reveals a new low-grade systolic murmur

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-100 fL)
- White blood cell count: 7,000/uL (Normal: 4,500-11,000/uL)
- Platelet count: 250,000/uL (Normal: 150,000-450,000/uL)
- Blood cultures: Positive for Streptococcus gallolyticus

## Encounter 3: source 2022-01-15, practice 2023-04-07, outpatient, Nephrology

Chief complaint: Follow-up visit for recently enrolled clinical trial for new medication to reduce progression of diabetic nephropathy.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Chen, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for recently enrolled clinical trial for new medication to reduce progression of diabetic nephropathy.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up regarding his recent enrollment in a clinical trial for a new medication aimed at reducing the progression of diabetic nephropathy, with history of diabetic neuropathy and colorectal neoplasia. His baseline serum creatinine 3 months ago was 1.8 mg/dL, and his urine albumin-to-creatinine ratio was 300 mg/g.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Colorectal neoplasia (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
His current medications include metformin, lisinopril, and the investigational drug.

Home Medications:
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

VITAL SIGNS:
Today, his blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 1.6 mg/dL
- Urine albumin-to-creatinine ratio: 250 mg/g
- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 138 mg/dL

## Encounter 4: source 2023-01-15, practice 2024-04-22, outpatient, Podiatry

Chief complaint: Non-healing wound on the plantar surface of the left foot present for two months with poorly controlled diabetes and numbness in feet.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Brown, MD
Department: Podiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Non-healing wound on the plantar surface of the left foot present for two months with poorly controlled diabetes and numbness in feet.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of a plantar foot wound in the context of long-standing diabetic sensorimotor polyneuropathy and ongoing clinical trial participation for diabetic nephropathy. He reports that the wound has been present for 2 months and has not improved with over-the-counter antibiotic ointments. His diabetes has been poorly controlled, with a recent hemoglobin A1c of 9.2%. He denies any fever or chills but notes numbness and tingling in his feet that has worsened over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Colorectal neoplasia (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, the patient has decreased sensation to light touch and pinprick in a stocking distribution. There is atrophy of the intrinsic muscles of the foot, and the toes on his left foot are contracted in a clawed position. The wound is located at the plantar aspect of the second metatarsal head and measures 2 cm in diameter with a calloused edge. Pulses are palpable in both feet.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Fasting blood glucose: 180 mg/dL
- Serum creatinine: 1.4 mg/dL
- Urinary albumin-to-creatinine ratio: 300 mg/g

## Encounter 5: source 2023-07-15, practice 2024-10-14, follow_up, Podiatry

Chief complaint: Follow-up visit for non-healing ulcer on left medial malleolus that has improved with regular debridement and wet-to-dry dressings over three months.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Michael Brown, MD
Department: Podiatry
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit for non-healing ulcer on left medial malleolus that has improved with regular debridement and wet-to-dry dressings over three months.

HISTORY OF PRESENT ILLNESS:
The patient is a 61-year-old male with a 15-year history of type 2 diabetes mellitus and peripheral neuropathy who returns for podiatry follow-up. He has a non-healing ulcer on his left medial malleolus that he first noticed 3 months ago. The ulcer initially had a moderate amount of slough and a foul odor, but after regular debridement and wet-to-dry dressings, the odor has resolved, and the amount of slough has significantly decreased.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Colorectal neoplasia (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On examination today, the ulcer measures 3 cm x 2 cm, with a clean base and healthy granulation tissue without any signs of infection. There is no surrounding erythema or warmth, and the patient reports a significant reduction in pain. His ankle-brachial index is 0.9.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- White blood cell count: 7.2 x 10^9/L
- C-reactive protein: 4 mg/L

## Encounter 6: source 2024-07-15, practice 2025-10-06, follow_up, General Surgery

Chief complaint: Follow-up visit three weeks after sustaining deep laceration to left forearm requiring primary suturing, with concerns about scar strength and return to work.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Sarah Kim, MD
Department: General Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit three weeks after sustaining deep laceration to left forearm requiring primary suturing, with concerns about scar strength and return to work.

HISTORY OF PRESENT ILLNESS:
Patient returns for follow-up of his forearm laceration in the context of long-standing diabetes and prior wound healing complications. He reports that the wound seems to be healing well, but he is concerned about the strength of the scar and his ability to return to work.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Colorectal neoplasia (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2023-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
He takes metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

REVIEW OF SYSTEMS:
He has no other complaints

PHYSICAL EXAMINATION:
On examination, the wound is clean with no signs of infection, and the sutures have been removed. There is a firm, raised scar along the site of the laceration

## Encounter 7: source 2025-01-15, practice 2026-04-07, ed, Emergency Medicine

Chief complaint: Acute two-day history of fever, headache, and right-sided facial pain following recent episode of diabetic ketoacidosis one month ago.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Roberto Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute two-day history of fever, headache, and right-sided facial pain following recent episode of diabetic ketoacidosis one month ago.

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old male with Type 2 Diabetes Mellitus, chronic kidney disease, and colorectal adenocarcinoma, enrolled in a clinical trial, who presents to the emergency department with a 2-day history of fever, headache, and right-sided facial pain. He reports a recent episode of diabetic ketoacidosis one month ago, which was managed in the hospital.

PAST MEDICAL HISTORY:
Active Problem List:
- Paget disease of bone (diagnosed 2020-01-15)
- Diabetic motor neuropathy (diagnosed 2020-04-15)
- Colorectal neoplasia (diagnosed 2021-01-15)
- Diabetic nephropathy (diagnosed 2022-01-15)
- Diabetic foot ulcer in proliferative phase of healing (diagnosed 2023-07-15)
- Wound healing, early maturation phase (3 weeks post-injury) (diagnosed 2024-07-15)
- Type 2 Diabetes Mellitus (15-year duration)
- Hypertension
- Chronic Kidney Disease Stage 3a secondary to diabetic nephropathy
- Peripheral Arterial Disease
- Diabetic Peripheral Neuropathy (sensorimotor polyneuropathy)
- Colorectal adenocarcinoma

PAST SURGICAL HISTORY:
- Primary suturing of left forearm deep laceration
- Hemicolectomy with primary anastomosis for colonic mass
- Multiple bedside debridements of left lower extremity ulcers

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Gabapentin 300 mg PO three times daily
- Investigational agent (clinical trial for diabetic nephropathy) per protocol

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, hypertension
- Paternal grandfather: Colorectal cancer (diagnosed age 65)
- Older brother: Paget disease of bone
- Mother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction laborer.

VITAL SIGNS:
His blood glucose level is 380 mg/dL.

PHYSICAL EXAMINATION:
On examination, the patient appears lethargic. He has periorbital edema and erythema on the right side, with black eschar noted on the nasal turbinates.

LABORATORY DATA:
Laboratory values are as follows:
- Hemoglobin A1c: 12%
- White blood cell count: 15,000/µL
- Blood urea nitrogen: 24 mg/dL
- Creatinine: 1.2 mg/dL
- Serum bicarbonate: 18 mEq/L

IMAGING:
A CT scan of the sinuses shows opacification and bony erosion of the right maxillary sinus.
