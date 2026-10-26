# synthetic_hospital patient 2650

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, Non-Hispanic White, Private (Aetna PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-28 (Tue) | outpatient | Gastroenterology |
| 1 | 2021-01-15 | 2026-01-31 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa drugs (rash)",
  "Shellfish (anaphylaxis)"
 ],
 "insurance": "Private (Aetna PPO)",
 "occupation": "Software developer",
 "alcohol_use": "Social (3-4 drinks per month)",
 "family_history": [
  "Father with primary hyperparathyroidism and pituitary macroadenoma (diagnosed age 48)",
  "Paternal grandmother with pancreatic neuroendocrine tumor"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg PO BID",
   "name": "Omeprazole"
  },
  {
   "dose": "500 mg PO TID with meals",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Extracorporeal shock wave lithotripsy (ESWL) for right renal calculus at age 26"
 ],
 "chronic_conditions": [
  "Multiple endocrine neoplasia type 1 (MEN1)",
  "Primary hyperparathyroidism",
  "Zollinger-Ellison syndrome",
  "Recurrent nephrolithiasis",
  "Refractory peptic ulcer disease"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Multiple endocrine neoplasia type 1", "Primary hyperparathyroidism"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-01-28, outpatient, Gastroenterology

Chief complaint: Three-week history of burning epigastric pain that worsens after eating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week history of burning epigastric pain that worsens after eating

HISTORY OF PRESENT ILLNESS:
He comes to the clinic with a three-week history of burning epigastric pain that worsens after eating. He was recently started on a proton pump inhibitor for refractory peptic ulcer disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple endocrine neoplasia type 1 (MEN1)
- Primary hyperparathyroidism
- Zollinger-Ellison syndrome
- Recurrent nephrolithiasis
- Refractory peptic ulcer disease

PAST SURGICAL HISTORY:
- Extracorporeal shock wave lithotripsy (ESWL) for right renal calculus at age 26

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO BID
- Calcium carbonate 500 mg PO TID with meals

ALLERGIES:
Sulfa drugs (rash), Shellfish (anaphylaxis)

FAMILY HISTORY:
Father having had surgery for a pituitary tumor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (3-4 drinks per month). Occupation: Software developer.

REVIEW OF SYSTEMS:
Intermittent headaches and mild visual changes

PHYSICAL EXAMINATION:
Mild abdominal tenderness and no focal neurological deficits

LABORATORY DATA:
Hypercalcemia and elevated parathyroid hormone

ASSESSMENT AND PLAN:
The patient asks about the likelihood that one of his future children will inherit his condition.

## Encounter 1: source 2021-01-15, practice 2026-01-31, ed, Emergency Medicine

Chief complaint: Sudden-onset severe shooting pain radiating from left flank into the groin

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe shooting pain radiating from left flank into the groin

HISTORY OF PRESENT ILLNESS:
The patient, a 27-year-old male with Multiple endocrine neoplasia type 1 initially evaluated in Gastroenterology for epigastric pain one year ago, presents acutely. He is visibly uncomfortable and cannot sit still on the stretcher. He denies dysuria, hematuria, or fever, but reports having had several similar episodes over the past year that resolved on their own. He has no prior history of trauma, recent illness, or heavy exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple endocrine neoplasia type 1 (diagnosed 2020-01-15)
- Multiple endocrine neoplasia type 1 (MEN1)
- Primary hyperparathyroidism
- Zollinger-Ellison syndrome
- Recurrent nephrolithiasis
- Refractory peptic ulcer disease

PAST SURGICAL HISTORY:
- Extracorporeal shock wave lithotripsy (ESWL) for right renal calculus at age 26

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO BID
- Calcium carbonate 500 mg PO TID with meals

ALLERGIES:
Sulfa drugs (rash), Shellfish (anaphylaxis)

FAMILY HISTORY:
His father and paternal aunt both had thyroid cancer requiring surgery

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (3-4 drinks per month). Occupation: Software developer.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, his abdomen is soft and non-tender, but there is left costovertebral angle tenderness. His thyroid exam is normal.

LABORATORY DATA:
Laboratory results are pending.

IMAGING:
A non-contrast CT of the abdomen and pelvis is ordered, which reveals a small left distal ureteral stone. The remainder of the imaging is unremarkable. A neck ultrasound performed during routine screening one year ago showed a stable hypoechoic thyroid nodule that had not been biopsied.
