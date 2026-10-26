# synthetic_hospital patient 1973

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, African American, Commercial Insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-25 (Thu) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-03-15 (Sat) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2025-11-07 (Fri) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial Insurance",
 "occupation": "Office Manager",
 "alcohol_use": "Occasional, 1-2 beers on weekends",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus, End-stage renal disease",
  "Mother: Hypertension, Stroke",
  "Brother: Type 2 Diabetes Mellitus"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, 15 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "500 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sitagliptin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "10 mg PO daily",
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
  "Appendectomy (complicated by perforation)",
  "Inguinal hernia repair (age 45)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Kidney Disease Stage 3b"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Acute appendicitis with perforation and septic shock", "Acute Hypercapnic Respiratory Failure", "Hyperosmolar Hyperglycemic State"]
Comorbidities: ["Lactic acidosis", "Acute kidney injury", "Type 2 diabetes mellitus", "Hyperkalemia", "Septic shock"]

## Encounter 0: source 2020-01-15, practice 2024-07-25, ed, Emergency Medicine

Chief complaint: 2-day history of worsening abdominal pain, fever, and vomiting with migration of pain from periumbilical area to right lower quadrant

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of worsening abdominal pain, fever, and vomiting with migration of pain from periumbilical area to right lower quadrant

HISTORY OF PRESENT ILLNESS:
He reports the pain began in the periumbilical area and has now localized to the right lower quadrant.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Appendectomy (complicated by perforation)
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Metformin 500 mg PO BID
- Sitagliptin 50 mg PO daily
- Amlodipine 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, End-stage renal disease
- Mother: Hypertension, Stroke
- Brother: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Office Manager.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/40 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 beats per minute.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes, decreased skin turgor, and right lower quadrant tenderness with rebound. On examination, the patient has bounding peripheral pulses.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/mm³
- Serum lactate: 4.2 mmol/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.5 mg/dL
- Sodium: 135 mEq/L
- Potassium: 4.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 18 mEq/L
- Glucose: 200 mg/dL

IMAGING:
Bedside ultrasound shows a small amount of free fluid in the abdomen.

## Encounter 1: source 2020-09-15, practice 2025-03-15, ed, Emergency Medicine

Chief complaint: 3-day history of polyuria, polydipsia, and progressive weakness with lethargy and confusion over the past day

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of polyuria, polydipsia, and progressive weakness with lethargy and confusion over the past day

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with Type 2 Diabetes Mellitus, Hypertension, Chronic Kidney Disease Stage 3b, and prior septic shock from perforated appendicitis presents with increasing thirst and polyuria. Over the past day, he has become lethargic and confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis with perforation and septic shock (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Appendectomy (complicated by perforation)
- Inguinal hernia repair (age 45)

MEDICATIONS:
His medications include metformin and sitagliptin.

Home Medications:
- Amlodipine 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, End-stage renal disease
- Mother: Hypertension, Stroke
- Brother: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Office Manager.

VITAL SIGNS:
On examination, his temperature is 37.2°C (99.0°F), blood pressure is 100/60 mmHg, pulse is 110/min, and respirations are 24/min with deep sighing breaths.

PHYSICAL EXAMINATION:
His skin is warm and dry. Neurological examination reveals altered mental status with no focal deficits.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum glucose: 680 mg/dL
- Serum sodium: 145 mEq/L
- Serum potassium: 5.2 mEq/L
- Serum bicarbonate: 18 mEq/L
- Blood urea nitrogen: 50 mg/dL
- Serum creatinine: 2.2 mg/dL
- Urinalysis: 4+ glucose, no ketones

## Encounter 2: source 2021-05-15, practice 2025-11-07, icu, Pulmonary & Critical Care

Chief complaint: 2-day history of fever, chills, and confusion with decreased responsiveness and failure to take medications over the past week

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
2-day history of fever, chills, and confusion with decreased responsiveness and failure to take medications over the past week

HISTORY OF PRESENT ILLNESS:
The patient with diabetes and prior Hyperosmolar Hyperglycemic State presents. His wife reports that he has been less responsive and has not been taking his medications or eating well for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute appendicitis with perforation and septic shock (diagnosed 2020-01-15)
- Hyperosmolar Hyperglycemic State (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease Stage 3b

PAST SURGICAL HISTORY:
- Appendectomy (complicated by perforation)
- Inguinal hernia repair (age 45)

MEDICATIONS:
Home Medications:
- Metformin 500 mg PO BID
- Sitagliptin 50 mg PO daily
- Amlodipine 5 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus, End-stage renal disease
- Mother: Hypertension, Stroke
- Brother: Type 2 Diabetes Mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional, 1-2 beers on weekends. Occupation: Office Manager.

VITAL SIGNS:
Respiratory rate of 30 breaths per minute, blood pressure of 90/60 mmHg

PHYSICAL EXAMINATION:
Lethargic, tachypneic, His skin is warm and flushed, The patient's respiratory effort is labored, and he appears to be using accessory muscles to breathe.

LABORATORY DATA:
- Serum sodium: 142 mEq/L
- Serum potassium: 5.2 mEq/L
- Serum chloride: 100 mEq/L
- Serum bicarbonate: 12 mEq/L
- Blood urea nitrogen: 50 mg/dL
- Serum creatinine: 3.5 mg/dL
- Serum glucose: 180 mg/dL
- Arterial blood gas (ABG):
- pH: 7.15
- PaCO2: 50 mmHg
- PaO2: 70 mmHg
- HCO3^-: 13 mEq/L
