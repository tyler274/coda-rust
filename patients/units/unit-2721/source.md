# synthetic_hospital patient 2721

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex F, Non-Hispanic White, Private (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-29 (Sun) | inpatient | Urology |
| 1 | 2020-07-15 | 2025-03-26 (Wed) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-03-19 (Thu) | inpatient | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private (Blue Cross Blue Shield)",
 "occupation": "Administrative Assistant",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
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
  "Laparoscopic cholecystectomy (2019)",
  "Cesarean section (1992)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension (poorly controlled)",
  "Obesity",
  "Recurrent Urinary Tract Infections"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Acute decompensated heart failure", "Acute laryngitis", "Perinephric abscess"]
Comorbidities: ["Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-09-29, inpatient, Urology

Chief complaint: Intermittent fever, chills, and left-sided flank pain for 2 weeks with 5-pound weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Urology
Visit Type: inpatient

CHIEF COMPLAINT:
Intermittent fever, chills, and left-sided flank pain for 2 weeks with 5-pound weight loss

HISTORY OF PRESENT ILLNESS:
2-week history of intermittent fever, chills, and left-sided flank pain. She reports a 5-pound weight loss over the same period. Her last documented urinary tract infection was treated with oral antibiotics 6 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension (poorly controlled)
- Obesity
- Recurrent Urinary Tract Infections

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2019)
- Cesarean section (1992)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Administrative Assistant.

VITAL SIGNS:
Temperature is 38.7°C (101.7°F), blood pressure is 135/85 mmHg, and heart rate is 98 beats per minute

PHYSICAL EXAMINATION:
Costovertebral angle tenderness on the left side

LABORATORY DATA:
White blood cell count: 14,000/mm³ (normal: 4,500-11,000/mm³)
- Hemoglobin: 12.2 g/dL (normal: 12-16 g/dL)
- Blood urea nitrogen: 18 mg/dL (normal: 7-20 mg/dL)
- Serum creatinine: 1.1 mg/dL (normal: 0.6-1.1 mg/dL)
- Urinalysis: Trace protein, 10-20 white blood cells/high-power field, negative nitrites, and no bacteria seen

IMAGING:
A contrast-enhanced computed tomography (CT) scan of the abdomen and pelvis shows a 3 cm fluid collection adjacent to the left kidney.

## Encounter 1: source 2020-07-15, practice 2025-03-26, ed, Emergency Medicine

Chief complaint: Fever, jaundice, and severe right upper quadrant abdominal pain radiating to the back for 2 days with dark urine and pale stools

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, jaundice, and severe right upper quadrant abdominal pain radiating to the back for 2 days with dark urine and pale stools

HISTORY OF PRESENT ILLNESS:
This 55-year-old female with Type 2 Diabetes Mellitus, poorly controlled hypertension, obesity, and recurrent urinary tract infections presents to the emergency department six months after hospitalization for a perinephric abscess, with a 2-day history of fever, jaundice, and severe right upper quadrant abdominal pain that radiates to the back. She reports that the pain is constant and has been worsening. She also notes dark urine and pale stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Perinephric abscess (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension (poorly controlled)
- Obesity
- Recurrent Urinary Tract Infections

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2019)
- Cesarean section (1992)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her temperature is 38.9°C (102°F), heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right upper quadrant with a positive Murphy's sign.

LABORATORY DATA:
White blood cell count: 16,000/mm³ with left shift
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Total bilirubin: 5.2 mg/dL (direct bilirubin 3.8 mg/dL)
- Alkaline phosphatase: 280 U/L
- Aspartate aminotransferase (AST): 80 U/L
- Alanine aminotransferase (ALT): 75 U/L

IMAGING:
An abdominal ultrasound shows dilation of the common bile duct and the presence of gallstones.

## Encounter 2: source 2021-07-15, practice 2026-03-19, inpatient, Cardiology

Chief complaint: Progressively worsening shortness of breath over 2 weeks with orthopnea requiring two pillows, bilateral leg swelling, and nonproductive cough

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressively worsening shortness of breath over 2 weeks with orthopnea requiring two pillows, bilateral leg swelling, and nonproductive cough

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with Type 2 Diabetes Mellitus, poorly controlled hypertension, obesity, and recurrent urinary tract infections returns following 2020 perinephric abscess and emergency department visit. She reports that walking up a single flight of stairs leaves her winded and now sleeps propped up on two pillows due to discomfort when lying flat, with mild bilateral leg swelling and nonproductive cough, but denies chest pain, hemoptysis, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Perinephric abscess (diagnosed 2020-01-15)
- Acute laryngitis (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension (poorly controlled)
- Obesity
- Recurrent Urinary Tract Infections

Her past medical history includes poorly controlled hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2019)
- Cesarean section (1992)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 10 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
Temperature is 36.9°C (98.4°F), blood pressure is 152/94 mm Hg, pulse is 94/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On physical examination, the patient appears mildly dyspneic at rest. Auscultation reveals diminished breath sounds and dullness to percussion over the right lower lung field. Tactile fremitus is reduced in the same area. Cardiac examination reveals an S3 gallop, and there is trace bilateral pitting edema in the lower extremities.

IMAGING:
Chest x-ray shows blunting of the right costophrenic angle.

OTHER STUDIES:
Thoracentesis yields clear, straw-colored fluid. Pleural fluid analysis shows:
Protein  2.4 g/dL
LDH      82 U/L
WBCs   350/µL
