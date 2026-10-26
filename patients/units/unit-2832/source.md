# synthetic_hospital patient 2832

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-09 (Wed) | inpatient | Orthopedic Surgery |
| 1 | 2020-07-15 | 2026-01-15 (Thu) | icu | Internal Medicine |
| 2 | 2020-07-15 | 2026-01-15 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Heparin (heparin-induced thrombocytopenia - type II)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine weekly)",
 "family_history": [
  "Father: Myocardial infarction at age 72, hypertension",
  "Mother: Ischemic stroke at age 75, atrial fibrillation",
  "Sister: Deep vein thrombosis (postpartum)",
  "No family history of bleeding disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 20 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg daily (target INR 2.0-3.0 for atrial fibrillation)",
   "name": "Warfarin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "600 mg/400 IU PO twice daily",
   "name": "Calcium carbonate with vitamin D"
  }
 ],
 "surgical_history": [
  "Total hip arthroplasty (right) - complicated by heparin-induced thrombocytopenia with arterial thrombosis",
  "Total knee arthroplasty (left) - complicated by heparin-induced thrombocytopenia with venous thromboembolism",
  "Cataract extraction (bilateral)",
  "Cholecystectomy (laparoscopic)"
 ],
 "chronic_conditions": [
  "Atrial fibrillation (non-valvular)",
  "Hypertension",
  "Osteoarthritis (bilateral hips and knees)",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Heparin-induced thrombocytopenia"]
Comorbidities: ["Atrial fibrillation", "Status post hip replacement surgery"]

## Encounter 0: source 2020-01-15, practice 2025-07-09, inpatient, Orthopedic Surgery

Chief complaint: Admitted for elective right hip replacement surgery, developed new left foot pain on post-operative day 3

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted for elective right hip replacement surgery, developed new left foot pain on post-operative day 3

HISTORY OF PRESENT ILLNESS:
As part of her pre-operative management, she was started on an anticoagulant to bridge therapy prior to stopping her chronic warfarin. On post-operative day 3, she develops new pain in her left foot.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (non-valvular)
- Hypertension
- Osteoarthritis (bilateral hips and knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Total hip arthroplasty (right) - complicated by heparin-induced thrombocytopenia with arterial thrombosis
- Total knee arthroplasty (left) - complicated by heparin-induced thrombocytopenia with venous thromboembolism
- Cataract extraction (bilateral)
- Cholecystectomy (laparoscopic)

MEDICATIONS:
She takes metoprolol and lisinopril; anticoagulant; chronic warfarin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate with vitamin D 600 mg/400 IU PO twice daily

ALLERGIES:
Heparin (heparin-induced thrombocytopenia - type II), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Ischemic stroke at age 75, atrial fibrillation
- Sister: Deep vein thrombosis (postpartum)
- No family history of bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Pallor in her left foot with diminished pulses

LABORATORY DATA:
Platelet count decreased from 240,000/μL at admission to 80,000/μL today. Her activated partial thromboplastin time (aPTT) and prothrombin time (PT) are within normal limits.

IMAGING:
Doppler ultrasound reveals an arterial thrombus in the left femoral artery.

## Encounter 1: source 2020-07-15, practice 2026-01-15, icu, Internal Medicine

Chief complaint: Sudden-onset shortness of breath and chest pain on post-operative day 4 following elective left knee replacement

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Park, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset shortness of breath and chest pain on post-operative day 4 following elective left knee replacement

HISTORY OF PRESENT ILLNESS:
Patient returns following knee replacement after hip surgery with heparin-induced thrombocytopenia. Postoperatively, she is started on heparin for venous thromboembolism prophylaxis. On postoperative day 4, she develops sudden-onset shortness of breath and chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Atrial fibrillation (non-valvular)
- Hypertension
- Osteoarthritis (bilateral hips and knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Total hip arthroplasty (right) - complicated by heparin-induced thrombocytopenia with arterial thrombosis
- Total knee arthroplasty (left) - complicated by heparin-induced thrombocytopenia with venous thromboembolism
- Cataract extraction (bilateral)
- Cholecystectomy (laparoscopic)

MEDICATIONS:
She takes on chronic anticoagulation with warfarin
started on heparin for venous thromboembolism prophylaxis.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate with vitamin D 600 mg/400 IU PO twice daily

ALLERGIES:
Heparin (heparin-induced thrombocytopenia - type II), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Ischemic stroke at age 75, atrial fibrillation
- Sister: Deep vein thrombosis (postpartum)
- No family history of bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 110 bpm, and oxygen saturation is 92% on room air.

LABORATORY DATA:
Her complete blood count shows a platelet count that has dropped from 250,000/mm3 at admission to 80,000/mm3.

IMAGING:
A chest CT angiogram reveals a segmental pulmonary embolism.

PLAN:
A heparin-induced thrombocytopenia (HIT) antibody test is ordered.

## Encounter 2: source 2020-07-15, practice 2026-01-15, ed, Emergency Medicine

Chief complaint: 2-day history of left calf pain and swelling 7 days after left knee replacement surgery

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Jennifer Lopez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of left calf pain and swelling 7 days after left knee replacement surgery

HISTORY OF PRESENT ILLNESS:
History of HIT and recent ICU admission. She reports that she has been receiving subcutaneous heparin injections since her surgery 7 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Heparin-induced thrombocytopenia (diagnosed 2020-01-15)
- Atrial fibrillation (non-valvular)
- Hypertension
- Osteoarthritis (bilateral hips and knees)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Total hip arthroplasty (right) - complicated by heparin-induced thrombocytopenia with arterial thrombosis
- Total knee arthroplasty (left) - complicated by heparin-induced thrombocytopenia with venous thromboembolism
- Cataract extraction (bilateral)
- Cholecystectomy (laparoscopic)

MEDICATIONS:
She takes subcutaneous heparin injections.

Home Medications:
- Warfarin 5 mg daily (target INR 2.0-3.0 for atrial fibrillation)
- Metoprolol succinate 50 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily
- Calcium carbonate with vitamin D 600 mg/400 IU PO twice daily

ALLERGIES:
Heparin (heparin-induced thrombocytopenia - type II), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Myocardial infarction at age 72, hypertension
- Mother: Ischemic stroke at age 75, atrial fibrillation
- Sister: Deep vein thrombosis (postpartum)
- No family history of bleeding disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (20 pack-year history). Alcohol: Social (1-2 glasses of wine weekly). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are stable.

PHYSICAL EXAMINATION:
On examination, her left calf is erythematous, warm, and tender to palpation.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
- White blood cell count: 7.8 x 10^9/L
- Platelet count: 55 x 10^9/L (baseline 150 x 10^9/L)
- Prothrombin time (PT): 12 seconds
- Activated partial thromboplastin time (aPTT): 35 seconds
- D-dimer: Elevated

IMAGING:
A Doppler ultrasound of the left leg reveals deep vein thrombosis.
