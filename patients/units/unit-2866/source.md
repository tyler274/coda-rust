# synthetic_hospital patient 2866

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental Plan G.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-22 (Sun) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2025-09-23 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Plan G",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "1-2 glasses of wine weekly",
 "family_history": [
  "Father: Atrial fibrillation, hypertension (deceased at 81)",
  "Mother: Type 2 diabetes, stroke at age 76",
  "Maternal uncle: Pulmonary embolism following hip surgery",
  "Sister: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 12 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily (held perioperatively)",
   "name": "Warfarin"
  },
  {
   "dose": "100 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg daily",
   "name": "Rosuvastatin"
  },
  {
   "dose": "650 mg every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Total knee arthroplasty (left) 7 days prior to first encounter",
  "Hysterectomy for uterine fibroids (1998)",
  "Appendectomy (1975)"
 ],
 "chronic_conditions": [
  "Chronic atrial fibrillation",
  "Hypertension",
  "Hyperlipidemia",
  "Osteoarthritis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Heparin-Induced Thrombocytopenia Type II", "Pulmonary embolism"]
Comorbidities: ["Chronic atrial fibrillation", "Status post hip replacement surgery"]

## Encounter 0: source 2020-01-15, practice 2024-09-22, ed, Emergency Medicine

Chief complaint: 2-day history of left calf pain and swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of left calf pain and swelling

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 2-day history of left calf pain and swelling. She reports no recent trauma or prolonged immobilization.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic atrial fibrillation
- Hypertension
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
Recent total knee replacement surgery

MEDICATIONS:
Her medications include warfarin, which was held perioperatively, and subcutaneous heparin, initiated 7 days ago for venous thromboembolism prophylaxis.

Home Medications:
- Metoprolol succinate 100 mg daily
- Lisinopril 10 mg daily
- Rosuvastatin 20 mg daily
- Acetaminophen 650 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Atrial fibrillation, hypertension (deceased at 81)
- Mother: Type 2 diabetes, stroke at age 76
- Maternal uncle: Pulmonary embolism following hip surgery
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 12 years ago. Alcohol: 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her vital signs are stable.

PHYSICAL EXAMINATION:
On examination, her left calf is erythematous, warm, and tender to palpation. The circumference of the left calf is 3 cm greater than the right.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 13.2 g/dL
- Platelet count: 55,000/µL (decreased from 250,000/µL measured preoperatively)
- White blood cell count: 7.8 x 10^3/µL
- Prothrombin time: 12 seconds
- Activated partial thromboplastin time: 28 seconds
- D-dimer: Elevated

IMAGING:
A Doppler ultrasound of the left leg reveals a deep vein thrombosis in the popliteal vein.

## Encounter 1: source 2021-01-15, practice 2025-09-23, ed, Emergency Medicine

Chief complaint: Sudden onset of sharp, right-sided chest pain that worsens with deep breaths

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of sharp, right-sided chest pain that worsens with deep breaths

HISTORY OF PRESENT ILLNESS:
Patient returns with chest pain one year after left calf pain. She also reports a productive cough with blood-tinged sputum and shortness of breath that started abruptly two hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Heparin-Induced Thrombocytopenia Type II (diagnosed 2020-01-15)
- Chronic atrial fibrillation
- Hypertension
- Hyperlipidemia
- Osteoarthritis

She has a history of chronic atrial fibrillation.

PAST SURGICAL HISTORY:
She was recently hospitalized for a total knee replacement four weeks ago

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (held perioperatively)
- Metoprolol succinate 100 mg daily
- Lisinopril 10 mg daily
- Rosuvastatin 20 mg daily
- Acetaminophen 650 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Atrial fibrillation, hypertension (deceased at 81)
- Mother: Type 2 diabetes, stroke at age 76
- Maternal uncle: Pulmonary embolism following hip surgery
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 12 years ago. Alcohol: 1-2 glasses of wine weekly. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Her heart rate is 110 bpm, respiratory rate is 22 breaths/min, blood pressure is 135/85 mmHg, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
A chest examination reveals decreased breath sounds at the right lung base. Her lower extremities show no signs of swelling or redness, but she reports tenderness on the right calf upon deep palpation.

LABORATORY DATA:
A D-dimer test is elevated.

IMAGING:
A chest CT scan with contrast shows a wedge-shaped area of increased opacity in the right lower lobe.
