# synthetic_hospital patient 2019

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-06 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2024-10-16 (Wed) | outpatient | Internal Medicine |
| 2 | 2021-05-15 | 2025-10-14 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Senior financial analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Hypertension and type 2 diabetes mellitus",
  "Older brother: Ischemic stroke at age 52"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
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
  }
 ],
 "surgical_history": [
  "Percutaneous left kidney biopsy (diagnostic)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Nephrotic syndrome secondary to membranous nephropathy"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Renal vein thrombosis", "Stroke", "Acute myocardial infarction"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-06-06, outpatient, Internal Medicine

Chief complaint: Follow-up visit after starting new medication for primary cardiovascular prevention

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit after starting new medication for primary cardiovascular prevention

HISTORY OF PRESENT ILLNESS:
He was recently started on a new medication for primary prevention of cardiovascular events. The patient has no complaints and is adherent to his medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Nephrotic syndrome secondary to membranous nephropathy

PAST SURGICAL HISTORY:
- Percutaneous left kidney biopsy (diagnostic)

MEDICATIONS:
His current medications include lisinopril, atorvastatin, and aspirin.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and type 2 diabetes mellitus
- Older brother: Ischemic stroke at age 52

SOCIAL HISTORY:
He does not smoke and

VITAL SIGNS:
His blood pressure today is 132/84 mmHg, and his heart rate is 78 bpm.
has a BMI of 27 kg/m^2.

LABORATORY DATA:
His recent lipid panel is as follows:

- Total cholesterol: 190 mg/dL
- HDL cholesterol: 45 mg/dL
- LDL cholesterol: 100 mg/dL
- Triglycerides: 225 mg/dL

ASSESSMENT AND PLAN:
A recent randomized controlled trial showed that the addition of the new medication to standard therapy in patients with similar profiles led to a 2% absolute risk reduction in myocardial infarction over a 5-year period. The rate of myocardial infarction in the control group was 8%, while it was 6% in the treatment group.

## Encounter 1: source 2020-05-15, practice 2024-10-16, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit for ongoing antihypertensive therapy management

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit for ongoing antihypertensive therapy management

HISTORY OF PRESENT ILLNESS:
The patient returns for antihypertensive follow-up with history of hypertension, hyperlipidemia, and nephrotic syndrome secondary to membranous nephropathy. He participates in a randomized controlled trial for a new antihypertensive medication and reports adherence to his medication regimen and low-sodium diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute myocardial infarction (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Nephrotic syndrome secondary to membranous nephropathy

PAST SURGICAL HISTORY:
- Percutaneous left kidney biopsy (diagnostic)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and type 2 diabetes mellitus
- Older brother: Ischemic stroke at age 52

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Senior financial analyst.

REVIEW OF SYSTEMS:
He has no complaints of chest pain, shortness of breath, or dizziness.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

OTHER STUDIES:
The trial's preliminary results show a relative risk (RR) of 0.85 (95% CI, 0.75-0.97) for the incidence of stroke in patients taking the new medication compared to those on standard treatment.

## Encounter 2: source 2021-05-15, practice 2025-10-14, ed, Emergency Medicine

Chief complaint: Acute onset left flank pain and gross hematuria with increasing left leg and scrotal swelling

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Marcus Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset left flank pain and gross hematuria with increasing left leg and scrotal swelling

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with hypertension, hyperlipidemia, nephrotic syndrome, and prior MI and stroke presents to the emergency department with a 2-day history of left flank pain and gross hematuria. He also complains of increasing swelling in his left leg and scrotum.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute myocardial infarction (diagnosed 2020-01-15)
- Stroke (diagnosed 2020-05-15)
- Hypertension
- Hyperlipidemia
- Nephrotic syndrome secondary to membranous nephropathy

His medical history is significant for hypertension and a recent diagnosis of nephrotic syndrome secondary to membranous nephropathy.

PAST SURGICAL HISTORY:
- Percutaneous left kidney biopsy (diagnostic)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and type 2 diabetes mellitus
- Older brother: Ischemic stroke at age 52

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Senior financial analyst.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, pulse is 88 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
His left leg is markedly swollen compared to the right, and there is pitting edema extending to the mid-thigh. The scrotum is also edematous.

LABORATORY DATA:
Laboratory results are as follows:

- Urinalysis: 3+ protein, RBCs too numerous to count
- Serum albumin: 2.1 g/dL
- Serum creatinine: 1.2 mg/dL
- D-dimer: Elevated
- Lipid profile: Cholesterol 300 mg/dL, LDL 190 mg/dL, HDL 30 mg/dL
