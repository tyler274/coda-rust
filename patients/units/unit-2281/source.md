# synthetic_hospital patient 2281

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Medicare primary, supplemental Medigap Plan F.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-13 (Thu) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-08-08 (Fri) | ed | Emergency Medicine |
| 2 | 2021-04-15 | 2026-05-05 (Tue) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare primary, supplemental Medigap Plan F",
 "occupation": "Retired postal worker (disability)",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus, end-stage renal disease (dialysis-dependent), deceased at age 71 from myocardial infarction",
  "Mother: Hypertension, chronic kidney disease, age 89 and living",
  "One sibling with Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 18 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "28 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale 1:10 ratio with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "500 mg PO twice daily (held if NPO or contrast exposure)",
   "name": "Metformin"
  },
  {
   "dose": "300 mg PO three times daily for neuropathy",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [
  "Cataract extraction, bilateral (2021)",
  "Cardiac catheterization with drug-eluting stent to LAD (2020)",
  "Inguinal herniorrhaphy (2010)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus with diabetic nephropathy and peripheral neuropathy",
  "Chronic Kidney Disease, Stage 3b (baseline serum creatinine 1.1-1.2 mg/dL)",
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Acute kidney injury", "Clostridium species infection (e.g., C. perfringens or C. difficile)", "Prerenal acute kidney injury secondary to hypovolemia"]
Comorbidities: ["Acute kidney injury", "Sepsis", "Septic shock"]

## Encounter 0: source 2020-01-15, practice 2025-02-13, ed, Emergency Medicine

Chief complaint: 2-day history of fever, chills, and confusion with decreased responsiveness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of fever, chills, and confusion with decreased responsiveness

HISTORY OF PRESENT ILLNESS:
His wife reports that he has been less responsive and has not been eating or drinking well.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus with diabetic nephropathy and peripheral neuropathy
- Chronic Kidney Disease, Stage 3b (baseline serum creatinine 1.1-1.2 mg/dL)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral (2021)
- Cardiac catheterization with drug-eluting stent to LAD (2020)
- Inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Insulin glargine 28 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:10 ratio with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily
- Metformin 500 mg PO twice daily (held if NPO or contrast exposure)
- Gabapentin 300 mg PO three times daily for neuropathy

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, end-stage renal disease (dialysis-dependent), deceased at age 71 from myocardial infarction
- Mother: Hypertension, chronic kidney disease, age 89 and living
- One sibling with Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/50 mmHg, heart rate is 110 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes and poor skin turgor.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12 g/dL
- Platelet count: 250,000/mm³
- Blood urea nitrogen (BUN): 60 mg/dL
- Serum creatinine: 2.0 mg/dL (baseline 1.2 mg/dL)
- Serum sodium: 135 mEq/L
- Serum potassium: 4.5 mEq/L
- Urinary sodium: 15 mEq/L
- Fractional excretion of sodium (FeNa): 0.8%

Urine microscopy shows a bland sediment. Blood cultures have been drawn and are pending.

## Encounter 1: source 2020-07-15, practice 2025-08-08, ed, Emergency Medicine

Chief complaint: Collapse at home preceded by progressive right foot swelling, pain, and discoloration after minor gardening injury

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse at home preceded by progressive right foot swelling, pain, and discoloration after minor gardening injury

HISTORY OF PRESENT ILLNESS:
The patient is a 64-year-old male with Type 2 DM and prior AKI in January 2020, presenting with right foot symptoms. His wife reports that over the past 24 hours, he developed progressive swelling, pain, and discoloration of his right foot following a minor puncture wound sustained while gardening two days ago. He initially dismissed the injury as trivial and did not seek medical attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy and peripheral neuropathy
- Chronic Kidney Disease, Stage 3b (baseline serum creatinine 1.1-1.2 mg/dL)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral (2021)
- Cardiac catheterization with drug-eluting stent to LAD (2020)
- Inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Insulin glargine 28 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:10 ratio with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily
- Metformin 500 mg PO twice daily (held if NPO or contrast exposure)
- Gabapentin 300 mg PO three times daily for neuropathy

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, end-stage renal disease (dialysis-dependent), deceased at age 71 from myocardial infarction
- Mother: Hypertension, chronic kidney disease, age 89 and living
- One sibling with Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

VITAL SIGNS:
On arrival, the patient is febrile to 39.3°C (102.7°F), hypotensive at 82/50 mmHg, and tachycardic at 124/min.

PHYSICAL EXAMINATION:
He appears lethargic and is oriented only to person. Physical exam reveals a tense, swollen right foot with bullae, violaceous skin discoloration, and areas of black necrosis. Palpation elicits crepitus, and the area is exquisitely tender.

LABORATORY DATA:
Initial labs show WBC count of 18,400/mm³, lactate of 6.1 mmol/L, and serum creatinine of 2.1 mg/dL.

IMAGING:
A stat X-ray of the foot reveals gas tracking through the soft tissues.

OTHER STUDIES:
Blood and wound cultures are pending.

PLAN:
The patient is started on aggressive IV fluids and broad-spectrum antibiotics while awaiting surgical consult.

## Encounter 2: source 2021-04-15, practice 2026-05-05, icu, Internal Medicine

Chief complaint: Sepsis and hypotension secondary to pneumonia with acute oliguria and rising creatinine on hospital day 4

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sepsis and hypotension secondary to pneumonia with acute oliguria and rising creatinine on hospital day 4

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Type 2 Diabetes Mellitus, diabetic nephropathy, and Chronic Kidney Disease Stage 3b with prior acute kidney injury, who returns for treatment of sepsis and hypotension secondary to pneumonia. On hospital day 4, his urine output drops markedly, and serum creatinine rises from 1.2 mg/dL at admission to 3.9 mg/dL. He is now oliguric.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Clostridium species infection (e.g., C. perfringens or C. difficile) (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus with diabetic nephropathy and peripheral neuropathy
- Chronic Kidney Disease, Stage 3b (baseline serum creatinine 1.1-1.2 mg/dL)
- Hypertension
- Hyperlipidemia
- Coronary artery disease

PAST SURGICAL HISTORY:
- Cataract extraction, bilateral (2021)
- Cardiac catheterization with drug-eluting stent to LAD (2020)
- Inguinal herniorrhaphy (2010)

MEDICATIONS:
Home Medications:
- Insulin glargine 28 units subcutaneous at bedtime
- Insulin lispro sliding scale 1:10 ratio with meals
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO at bedtime
- Aspirin 81 mg PO daily
- Metformin 500 mg PO twice daily (held if NPO or contrast exposure)
- Gabapentin 300 mg PO three times daily for neuropathy

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, end-stage renal disease (dialysis-dependent), deceased at age 71 from myocardial infarction
- Mother: Hypertension, chronic kidney disease, age 89 and living
- One sibling with Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 18 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired postal worker (disability).

PHYSICAL EXAMINATION:
Physical examination reveals mild peripheral edema.

LABORATORY DATA:
Urinalysis shows muddy brown granular casts.
