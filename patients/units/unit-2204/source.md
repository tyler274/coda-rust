# synthetic_hospital patient 2204

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare Part A and B.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-18 (Tue) | inpatient | Internal Medicine |
| 1 | 2020-05-15 | 2025-07-12 (Sat) | ed | Emergency Medicine |
| 2 | 2020-11-15 | 2026-01-17 (Sat) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)"
 ],
 "insurance": "Medicare Part A and B",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "Father: Type 2 diabetes mellitus, stroke",
  "Mother: Hypertension, chronic kidney disease",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Glipizide"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Metoprolol"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Appendectomy, age 12",
  "Cataract surgery, age 62"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Chronic kidney disease, stage 3b",
  "Hypertension",
  "Diabetic foot ulcer, right plantar surface"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Acute pyelonephritis", "Septic shock", "Allergic reaction to cefazolin due to penicillin cross-sensitivity"]
Comorbidities: ["Acute kidney injury", "Fever", "Essential hypertension", "Type 2 diabetes mellitus", "Chronic kidney disease stage 4"]

## Encounter 0: source 2020-01-15, practice 2025-03-18, inpatient, Internal Medicine

Chief complaint: Management of a chronic right plantar diabetic foot ulcer requiring intravenous antibiotics and insulin therapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Anita Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Management of a chronic right plantar diabetic foot ulcer requiring intravenous antibiotics and insulin therapy

HISTORY OF PRESENT ILLNESS:
He is started on intravenous antibiotics and insulin therapy. His blood glucose levels are closely monitored, and his hypertension is controlled with oral metoprolol. On the third day of admission, the night shift nurse receives a signout from the day shift nurse that includes the patient&#x27;s demographics, medical history, and current treatment plan. However, the signout omits the patient&#x27;s allergy to penicillin and the recent adjustment of his insulin regimen due to episodes of nocturnal hypoglycemia. At midnight, the patient develops a fever and the on-call physician orders cefazolin before reviewing the patient&#x27;s allergy history. The patient subsequently develops a rash and pruritus. The next morning, the patient is found to have a blood glucose of 48 mg/dL.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Chronic kidney disease, stage 3b
- Hypertension
- Diabetic foot ulcer, right plantar surface

PAST SURGICAL HISTORY:
- Appendectomy, age 12
- Cataract surgery, age 62

MEDICATIONS:
He takes intravenous antibiotics, insulin therapy, oral metoprolol, cefazolin.

Home Medications:
- Glipizide 5 mg daily
- Lisinopril 10 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily

ALLERGIES:
Penicillin

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, stroke
- Mother: Hypertension, chronic kidney disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
Blood pressure: 132/86 mmHg
blood glucose of 48 mg/dL

LABORATORY DATA:
Laboratory values from the evening prior to the event:
- Hemoglobin A1c: 8.2%
- Fasting blood glucose: 156 mg/dL
- Postprandial blood glucose: 198 mg/dL

## Encounter 1: source 2020-05-15, practice 2025-07-12, ed, Emergency Medicine

Chief complaint: Three-day history of fever, chills, and right flank pain with burning sensation during urination and increased urinary frequency

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Kwame Osei, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of fever, chills, and right flank pain with burning sensation during urination and increased urinary frequency

HISTORY OF PRESENT ILLNESS:
Patient with diabetes, CKD 3b, cefazolin allergy reports burning sensation during urination and increased urinary frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic reaction to cefazolin due to penicillin cross-sensitivity (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease, stage 3b
- Hypertension
- Diabetic foot ulcer, right plantar surface

PAST SURGICAL HISTORY:
- Appendectomy, age 12
- Cataract surgery, age 62

MEDICATIONS:
Home Medications:
- Glipizide 5 mg daily
- Metoprolol 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, stroke
- Mother: Hypertension, chronic kidney disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 150/90 mmHg, pulse is 98/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
Physical examination reveals right costovertebral angle tenderness.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000/mm³
- Hemoglobin: 13.2 g/dL
- Blood urea nitrogen: 40 mg/dL
- Serum creatinine: 2.2 mg/dL (baseline 1.6 mg/dL)
- Urinalysis: Pyuria, positive leukocyte esterase, and nitrites
Two days later, the urine culture grows Escherichia coli resistant to multiple antibiotics, but sensitive to aminoglycosides.

PLAN:
A urine culture is obtained, and the patient is started on intravenous fluids and empiric antibiotics.
Given the patient's renal function and the sensitivity pattern, the decision is made to start an aminoglycoside.

## Encounter 2: source 2020-11-15, practice 2026-01-17, icu, Pulmonary & Critical Care

Chief complaint: Two-day history of fever, chills, and confusion with decreased responsiveness and poor oral intake

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Jennifer Wu, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Two-day history of fever, chills, and confusion with decreased responsiveness and poor oral intake

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with Type 2 diabetes mellitus, chronic kidney disease stage 3b, hypertension, and history of right plantar diabetic foot ulcer complicated by cefazolin allergy (January 2020) and acute pyelonephritis (May 2020) presents with decreased responsiveness and poor oral intake. His wife reports that he has been less responsive and has not been eating well.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic reaction to cefazolin due to penicillin cross-sensitivity (diagnosed 2020-01-15)
- Acute pyelonephritis (diagnosed 2020-05-15)
- Type 2 diabetes mellitus
- Chronic kidney disease, stage 3b
- Hypertension
- Diabetic foot ulcer, right plantar surface

PAST SURGICAL HISTORY:
- Appendectomy, age 12
- Cataract surgery, age 62

MEDICATIONS:
Home Medications:
- Glipizide 5 mg daily
- Metoprolol 50 mg twice daily
- Lisinopril 10 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, stroke
- Mother: Hypertension, chronic kidney disease
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 8 years ago. Alcohol: Social, 1-2 drinks per month. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 39.2°C, blood pressure is 85/50 mmHg, heart rate is 120 beats/min, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes and poor skin turgor. He is lethargic but arousable.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12 g/dL
- Platelet count: 150,000/mm³
- Serum creatinine: 2.5 mg/dL (baseline 1.6 mg/dL)
- Blood urea nitrogen: 50 mg/dL
- Lactic acid: 5 mmol/L
- Urinalysis: cloudy, positive leukocyte esterase, >50 white blood cells per high power field

OTHER STUDIES:
Blood cultures have been drawn.

PLAN:
After applying supplemental oxygen,
