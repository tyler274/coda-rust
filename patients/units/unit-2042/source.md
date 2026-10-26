# synthetic_hospital patient 2042

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-20 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-06-14 (Sat) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-03-09 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Warehouse logistics associate",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus with diabetic nephropathy",
  "Mother: Hypertension and hyperlipidemia",
  "Paternal uncle: Myocardial infarction at age 55"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Soft tissue infection", "Renal vein thrombosis"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-03-20, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up

HISTORY OF PRESENT ILLNESS:
He reports no new symptoms and is adherent to his medications. The patient is interested in the results of a recent clinical trial he heard about on the news, which compared the rates of myocardial infarction in patients taking a new lipid-lowering drug versus those on standard therapy. The 95% confidence interval for the difference in rates between the two groups was reported as -2% to 3%. Based on this information,

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He is currently taking lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy
- Mother: Hypertension and hyperlipidemia
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Warehouse logistics associate.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His recent laboratory results are as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL
- Fasting blood glucose: 88 mg/dL

## Encounter 1: source 2020-04-15, practice 2025-06-14, ed, Emergency Medicine

Chief complaint: comes to the emergency department because of fever and worsening pain in his right lower leg for the past 2 days

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He comes to the emergency department because of fever and worsening pain in his right lower leg for the past 2 days

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with known Type 2 diabetes mellitus, hypertension, and hyperlipidemia presents to the emergency department three months after his last routine clinic encounter. Five days ago, he scraped his shin while moving heavy boxes at work. The wound initially seemed minor, but over the past 48 hours, he has developed increasing redness, warmth, and swelling around the site. He has also experienced subjective fevers, chills, and malaise.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia

His medical history includes type 2 diabetes mellitus, hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy
- Mother: Hypertension and hyperlipidemia
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
On examination, his temperature is 38.9°C (102°F), blood pressure is 126/74 mm Hg, and pulse is 104/min.

PHYSICAL EXAMINATION:
The right anterior shin has an erythematous, tender area measuring 8 cm with surrounding edema and no fluctuance.

LABORATORY DATA:
Laboratory studies show:
White blood cell count 18,600/mm³
Segmented neutrophils 74%
Band forms 14%
Lymphocytes 8%
Monocytes 4%
Blood cultures are pending.

## Encounter 2: source 2021-01-15, practice 2026-03-09, ed, Emergency Medicine

Chief complaint: presents to the emergency department with a 2-day history of left-sided flank pain and swelling of his lower extremities

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He presents to the emergency department with a 2-day history of left-sided flank pain and swelling of his lower extremities

HISTORY OF PRESENT ILLNESS:
The patient with type 2 diabetes mellitus, hypertension, and hyperlipidemia returns to the emergency department nine months after his prior visit for soft tissue infection. He reports a 3-week history of increasing fatigue and frothy urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Soft tissue infection (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia

His past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes which are managed with metformin and lisinopril, respectively.

Home Medications:
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus with diabetic nephropathy
- Mother: Hypertension and hyperlipidemia
- Paternal uncle: Myocardial infarction at age 55

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Warehouse logistics associate.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
He has bilateral lower extremity pitting edema up to the level of his knees

LABORATORY DATA:
Laboratory findings are as follows:

- Serum albumin: 2.1 g/dL (Normal: 3.5-5.0 g/dL)
- Urine protein: 4+ on dipstick
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.2 mg/dL)
- Urine RBC: 0-2 per high power field
- Platelet count: 450,000/mm³ (Normal: 150,000-400,000/mm³)
- LDL cholesterol: 210 mg/dL (Normal: <100 mg/dL)

IMAGING:
A renal ultrasound shows loss of normal renal vein echogenicity and dilation of the renal vein.
