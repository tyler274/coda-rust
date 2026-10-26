# synthetic_hospital patient 2766

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Private employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-17 (Tue) | ed | Emergency Medicine |
| 1 | 2020-05-15 | 2025-10-19 (Sun) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-02-14 (Sat) | inpatient | Urology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Trimethoprim-sulfamethoxazole (Stevens-Johnson syndrome)",
  "Latex"
 ],
 "insurance": "Private employer-sponsored PPO",
 "occupation": "Remote data analyst",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Recurrent nephrolithiasis",
  "Father: Essential hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 2015, 10 pack-years)",
 "home_medications": [
  {
   "dose": "5 mg PO three times daily",
   "name": "Oxybutynin"
  },
  {
   "dose": "1 g PO twice daily",
   "name": "Methenamine hippurate"
  },
  {
   "dose": "0.4 mg PO daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "100 mg PO twice daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "Percutaneous nephrolithotomy (right kidney), 2020",
  "Ureteral stent placement/exchanges (multiple)",
  "Appendectomy, 2005"
 ],
 "chronic_conditions": [
  "Neurogenic bladder (secondary to T12 spinal cord injury)",
  "Recurrent urinary tract infections",
  "Chronic Proteus mirabilis bacteriuria"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Urolithiasis (Renal Colic)"]
Comorbidities: ["Urinary tract infection"]

## Encounter 0: source 2020-01-15, practice 2025-06-17, ed, Emergency Medicine

Chief complaint: Right flank pain and dysuria for 2 days

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Right flank pain and dysuria for 2 days

HISTORY OF PRESENT ILLNESS:
2-day history of right flank pain and dysuria. She reports the pain is constant and radiates to the groin. She also notes a frequent urge to urinate, but only small amounts of cloudy urine are passed. She denies any fever, nausea, or vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurogenic bladder (secondary to T12 spinal cord injury)
- Recurrent urinary tract infections
- Chronic Proteus mirabilis bacteriuria

PAST SURGICAL HISTORY:
- Percutaneous nephrolithotomy (right kidney), 2020
- Ureteral stent placement/exchanges (multiple)
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Oxybutynin 5 mg PO three times daily
- Methenamine hippurate 1 g PO twice daily
- Tamsulosin 0.4 mg PO daily
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (Stevens-Johnson syndrome), Latex

FAMILY HISTORY:
- Mother: Recurrent nephrolithiasis
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Former smoker (quit 2015, 10 pack-years). Alcohol: None. Occupation: Remote data analyst.

PHYSICAL EXAMINATION:
Costovertebral angle tenderness is present on the right side

LABORATORY DATA:
Urinalysis reveals a pH of 8.5, pyuria, and the presence of crystals. No bacteria are seen on Gram stain.

## Encounter 1: source 2020-05-15, practice 2025-10-19, ed, Emergency Medicine

Chief complaint: Fever, flank pain, and dysuria

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, flank pain, and dysuria

PAST MEDICAL HISTORY:
Active Problem List:
- Urolithiasis (Renal Colic) (diagnosed 2020-01-15)
- Neurogenic bladder (secondary to T12 spinal cord injury)
- Recurrent urinary tract infections
- Chronic Proteus mirabilis bacteriuria

She has a history of recurrent urinary tract infections and has been using an indwelling urinary catheter for the past several weeks due to a chronic neurogenic bladder.

PAST SURGICAL HISTORY:
- Percutaneous nephrolithotomy (right kidney), 2020
- Ureteral stent placement/exchanges (multiple)
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Oxybutynin 5 mg PO three times daily
- Methenamine hippurate 1 g PO twice daily
- Tamsulosin 0.4 mg PO daily
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (Stevens-Johnson syndrome), Latex

FAMILY HISTORY:
- Mother: Recurrent nephrolithiasis
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Former smoker (quit 2015, 10 pack-years). Alcohol: None. Occupation: Remote data analyst.

PHYSICAL EXAMINATION:
Physical examination reveals costovertebral angle tenderness on the right.

LABORATORY DATA:
Urinalysis shows alkaline urine and microscopy reveals rectangular prism-shaped crystals.

IMAGING:
Her kidney x-ray is shown below.

## Encounter 2: source 2020-09-15, practice 2026-02-14, inpatient, Urology

Chief complaint: 2-week history of right flank pain and intermittent fevers

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: inpatient

CHIEF COMPLAINT:
2-week history of right flank pain and intermittent fevers

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old female with neurogenic bladder secondary to T12 spinal cord injury, recurrent urinary tract infections, and chronic Proteus mirabilis bacteriuria who returns for further management following presentations to the emergency department in January and May 2020 for similar complaints. She reports a burning sensation during urination and a frequent urge to urinate.

PAST MEDICAL HISTORY:
Active Problem List:
- Urolithiasis (Renal Colic) (diagnosed 2020-01-15)
- Neurogenic bladder (secondary to T12 spinal cord injury)
- Recurrent urinary tract infections
- Chronic Proteus mirabilis bacteriuria

Her past medical history is significant for recurrent urinary tract infections, often with cultures positive for Proteus mirabilis.

PAST SURGICAL HISTORY:
- Percutaneous nephrolithotomy (right kidney), 2020
- Ureteral stent placement/exchanges (multiple)
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Oxybutynin 5 mg PO three times daily
- Methenamine hippurate 1 g PO twice daily
- Tamsulosin 0.4 mg PO daily
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (Stevens-Johnson syndrome), Latex

FAMILY HISTORY:
- Mother: Recurrent nephrolithiasis
- Father: Essential hypertension

SOCIAL HISTORY:
Smoking: Former smoker (quit 2015, 10 pack-years). Alcohol: None. Occupation: Remote data analyst.

VITAL SIGNS:
Her temperature is 38.3°C (100.9°F)

PHYSICAL EXAMINATION:
Costovertebral angle tenderness is noted on the right side.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000/mm³
- Urinalysis: pH 8.0, leukocyte esterase positive, nitrite positive, numerous white blood cells
- Urine culture: Pending
- Serum creatinine: 1.2 mg/dL (baseline 0.9 mg/dL)
- Blood urea nitrogen: 18 mg/dL

IMAGING:
A non-contrast CT scan of the abdomen and pelvis reveals a large, staghorn calculus in the right renal pelvis.

ASSESSMENT AND PLAN:
Given the patient's history and current presentation,
