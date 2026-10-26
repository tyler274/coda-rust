# synthetic_hospital patient 2195

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-05 (Fri) | inpatient | Nephrology |
| 1 | 2020-07-15 | 2026-03-13 (Fri) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-03-13 (Fri) | inpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired automotive mechanic",
 "alcohol_use": "Social, 1-2 beers on weekends",
 "family_history": [
  "Father: Myocardial infarction at age 71, hypertension",
  "Mother: Type 2 diabetes mellitus, chronic kidney disease",
  "Older brother: Hypertension, benign prostatic hyperplasia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago, 25 pack-year history",
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
   "dose": "0.4 mg PO daily at bedtime",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2008)",
  "Left cataract extraction with intraocular lens placement (2021)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Chronic Kidney Disease Stage 3a",
  "Benign Prostatic Hyperplasia"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Severe Clostridioides difficile infection without toxic megacolon", "Acute kidney injury"]
Comorbidities: ["Acute kidney injury", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-09-05, inpatient, Nephrology

Chief complaint: 2-day history of anuria accompanied by gradual decrease in urinary stream and nocturia over the past 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
2-day history of anuria accompanied by gradual decrease in urinary stream and nocturia over the past 6 months.

HISTORY OF PRESENT ILLNESS:
He reports a gradual decrease in urinary stream and nocturia over the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3a
- Benign Prostatic Hyperplasia

His past medical history includes hypertension, type 2 diabetes mellitus, and benign prostatic hyperplasia (BPH).

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2008)
- Left cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
His medications include metformin, lisinopril, and tamsulosin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hypertension
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Older brother: Hypertension, benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired automotive mechanic.

VITAL SIGNS:
His blood pressure is 150/90 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
On physical examination, he appears in mild distress. Abdominal examination reveals a distended bladder. Digital rectal examination shows an enlarged, firm, and non-tender prostate.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatinine: 3.2 mg/dL (baseline 1.0 mg/dL)
- Blood urea nitrogen (BUN): 45 mg/dL
- Serum potassium: 5.1 mEq/L
- Serum sodium: 138 mEq/L
- Urinalysis: No hematuria, proteinuria, or glucose

IMAGING:
A renal ultrasound is performed, which shows bilateral hydronephrosis without evidence of stones.

## Encounter 1: source 2020-07-15, practice 2026-03-13, ed, Emergency Medicine

Chief complaint: 3-day history of productive cough, fever, and shortness of breath; denies recent travel or sick contacts.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of productive cough, fever, and shortness of breath; denies recent travel or sick contacts.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with hypertension, type 2 diabetes, CKD stage 3a, and BPH returns after AKI hospitalization with a 3-day history of productive cough, fever, and shortness of breath. He denies recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3a
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2008)
- Left cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hypertension
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Older brother: Hypertension, benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired automotive mechanic.

VITAL SIGNS:
His temperature is 38.7°C (101.7°F), blood pressure is 135/85 mmHg, pulse is 102 beats/min, and respiratory rate is 22 breaths/min. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals crackles in the right lower lung field.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 15,000/µL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/µL
- Blood urea nitrogen: 28 mg/dL
- Serum creatinine: 1.2 mg/dL
- Sodium: 135 mEq/L
- Glucose: 180 mg/dL

IMAGING:
A chest radiograph shows a right lower lobe infiltrate.

PLAN:
The patient is admitted to the medical floor.

## Encounter 2: source 2020-07-15, practice 2026-03-13, inpatient, Gastroenterology

Chief complaint: 3-day history of voluminous watery diarrhea, abdominal cramping, and low-grade fever following recent antibiotic exposure.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
3-day history of voluminous watery diarrhea, abdominal cramping, and low-grade fever following recent antibiotic exposure.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with hypertension, type 2 diabetes mellitus, chronic kidney disease stage 3a, and benign prostatic hyperplasia, who required hospitalization for acute kidney injury in January, presents with a 3-day history of voluminous watery diarrhea, abdominal cramping, and low-grade fever following discharge 10 days ago for community-acquired pneumonia treated with ceftriaxone and azithromycin.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute kidney injury (diagnosed 2020-01-15)
- Opportunistic pulmonary infection requiring diagnostic sampling (diagnosed 2020-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Chronic Kidney Disease Stage 3a
- Benign Prostatic Hyperplasia

His past medical history includes hypertension, type 2 diabetes mellitus, and chronic kidney disease.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2008)
- Left cataract extraction with intraocular lens placement (2021)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Tamsulosin 0.4 mg PO daily at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 71, hypertension
- Mother: Type 2 diabetes mellitus, chronic kidney disease
- Older brother: Hypertension, benign prostatic hyperplasia

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago, 25 pack-year history. Alcohol: Social, 1-2 beers on weekends. Occupation: Retired automotive mechanic.

VITAL SIGNS:
Temperature of 37.8°C, blood pressure is 145/90 mmHg, and heart rate is 92 beats per minute

PHYSICAL EXAMINATION:
On physical examination, he is afebrile. Abdominal examination reveals mild diffuse tenderness without rebound or guarding. Bowel sounds are hyperactive.

LABORATORY DATA:
White blood cell count: 15,000 /µL (normal: 4,500-11,000 /µL)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Creatinine: 2.1 mg/dL (baseline 1.6 mg/dL)
- C-reactive protein: 25 mg/L (normal: <10 mg/L)
