# synthetic_hospital patient 2197

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-11 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2025-10-06 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-08-15 | 2026-03-16 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired warehouse supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 72",
  "Mother: Hypertension, cerebrovascular accident at age 78",
  "One sibling with hypertension and hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 8 years ago",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Left knee arthroscopic debridement 2018",
  "Cataract extraction with intraocular lens placement 2021"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "Chronic kidney disease Stage 3 (baseline serum creatinine 1.4 mg/dL)",
  "Osteoarthritis of bilateral knees"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Hyperosmolar Hyperglycemic State", "Acute kidney injury"]
Comorbidities: ["Acute kidney injury", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-08-11, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit for chronic conditions; patient reports new-onset persistent cough and shortness of breath over the past two weeks.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit for chronic conditions; patient reports new-onset persistent cough and shortness of breath over the past two weeks.

HISTORY OF PRESENT ILLNESS:
He reports a new-onset, persistent cough and shortness of breath that has worsened over the past two weeks. He denies chest pain, fever, or weight loss. The patient's electronic medical record reveals that his serum creatinine level was 1.4 mg/dL one month ago when he was seen for a similar complaint, and the physician increased the dose of lisinopril. The patient was not informed of the creatinine elevation at that time.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Chronic kidney disease Stage 3 (baseline serum creatinine 1.4 mg/dL)
- Osteoarthritis of bilateral knees

PAST SURGICAL HISTORY:
- Left knee arthroscopic debridement 2018
- Cataract extraction with intraocular lens placement 2021

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, cerebrovascular accident at age 78
- One sibling with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 78 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles at the lung bases.

LABORATORY DATA:
- Serum creatinine: 2.1 mg/dL (increased from 1.2 mg/dL six months ago)
- Serum potassium: 5.5 mEq/L
- B-type natriuretic peptide (BNP): 250 pg/mL

IMAGING:
- Chest X-ray: Mild cardiomegaly and bilateral pleural effusions

## Encounter 1: source 2020-03-15, practice 2025-10-06, outpatient, Internal Medicine

Chief complaint: Reduced appetite, mild fatigue, decreased urine output, and progressive lower extremity swelling over the past two weeks; worsening chronic bilateral knee pain aggravated by walking.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Reduced appetite, mild fatigue, decreased urine output, and progressive lower extremity swelling over the past two weeks; worsening chronic bilateral knee pain aggravated by walking.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with Type 2 diabetes mellitus, hypertension, hyperlipidemia, chronic kidney disease Stage 3, and osteoarthritis of the bilateral knees who returns for follow-up two months after a prior episode of acute kidney injury. He presents with reduced appetite, mild fatigue, decreased urine output, and progressive lower extremity swelling over the past two weeks. He also reports chronic bilateral knee pain that has been worsening over the last several months and is aggravated by walking or standing. He has no fever, dysuria, hematuria, or recent illness. He denies recent travel, nephrotoxic exposures, or new supplements.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Chronic kidney disease Stage 3 (baseline serum creatinine 1.4 mg/dL)
- Osteoarthritis of bilateral knees

His medical history is notable for multiple chronic conditions, including cardiovascular and metabolic comorbidities.

PAST SURGICAL HISTORY:
- Left knee arthroscopic debridement 2018
- Cataract extraction with intraocular lens placement 2021

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, cerebrovascular accident at age 78
- One sibling with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
Vital signs are within normal limits with a blood pressure of 124/76 mm Hg and a pulse of 68/min.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral pitting edema up to the mid-shins. Jugular venous distension is not present. Cardiac and pulmonary examinations are unremarkable. There is moderate bony tenderness of both knees without erythema, warmth, or joint effusion. Abdominal examination reveals no palpable masses or organomegaly. Neurologic examination is nonfocal. Skin shows no rashes, petechiae, or purpura.

LABORATORY DATA:
Laboratory studies show:
Creatinine 2.1 mg/dL
Blood urea nitrogen 49 mg/dL
Potassium 5.2 mEq/L

## Encounter 2: source 2020-08-15, practice 2026-03-16, ed, Emergency Medicine

Chief complaint: Acute altered mental status; family reports patient has been increasingly confused and lethargic over the past two days.

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute altered mental status; family reports patient has been increasingly confused and lethargic over the past two days.

HISTORY OF PRESENT ILLNESS:
Patient with CKD and prior AKI presents. His family reports that he has been increasingly confused over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Hyperlipidemia
- Chronic kidney disease Stage 3 (baseline serum creatinine 1.4 mg/dL)
- Osteoarthritis of bilateral knees

PAST SURGICAL HISTORY:
- Left knee arthroscopic debridement 2018
- Cataract extraction with intraocular lens placement 2021

MEDICATIONS:
Metformin, lisinopril, and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, cerebrovascular accident at age 78
- One sibling with hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 8 years ago. Alcohol: None. Occupation: Retired warehouse supervisor.

VITAL SIGNS:
Temperature is 37.2°C (99.0°F), blood pressure is 160/90 mmHg, pulse is 110/min, and respirations are 22/min

PHYSICAL EXAMINATION:
On examination, he is lethargic but arousable, with a Glasgow Coma Scale score of 14. His skin is warm and dry. Neurological examination reveals no focal deficits.

LABORATORY DATA:
- Glucose: 720 mg/dL
- Serum sodium: 145 mEq/L
- Serum potassium: 5.2 mEq/L
- Serum chloride: 100 mEq/L
- Serum bicarbonate: 18 mEq/L
- Blood urea nitrogen: 50 mg/dL
- Serum creatinine: 2.2 mg/dL
- Plasma osmolality: 330 mOsm/kg
- Urinalysis: 4+ glucose, no ketones
