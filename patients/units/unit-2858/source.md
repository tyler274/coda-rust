# synthetic_hospital patient 2858

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-12 (Sat) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-10-12 (Sun) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (rash)",
  "Penicillin (anaphylaxis)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Cholelithiasis with cholecystectomy age 68",
  "Father: Atrial fibrillation, hyperlipidemia, died of myocardial infarction age 72",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (15 pack-years, quit 8 years ago)",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Warfarin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy (2010)",
  "Right breast lumpectomy (2014)"
 ],
 "chronic_conditions": [
  "Chronic cholecystitis",
  "Atrial fibrillation",
  "Hyperlipidemia",
  "Hypertension"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Acute cholecystitis", "Gallstone ileus"]
Comorbidities: ["Chronic cholecystitis"]

## Encounter 0: source 2020-01-15, practice 2025-04-12, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant pain radiating to the back for 6 hours with nausea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant pain radiating to the back for 6 hours with nausea

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 6-hour history of severe right upper quadrant pain that radiates to her back. She reports that the pain was sudden in onset and is associated with nausea but no vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic cholecystitis
- Atrial fibrillation
- Hyperlipidemia
- Hypertension

Her past medical history is significant for hyperlipidemia and chronic cholecystitis.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (2010)
- Right breast lumpectomy (2014)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Penicillin (anaphylaxis)

FAMILY HISTORY:
- Mother: Cholelithiasis with cholecystectomy age 68
- Father: Atrial fibrillation, hyperlipidemia, died of myocardial infarction age 72
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
She denies any history of alcohol use or recent travel.

VITAL SIGNS:
On examination, her temperature is 38.2°C (100.8°F), blood pressure is 135/85 mmHg, pulse is 98/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right upper quadrant with a positive Murphy's sign.

LABORATORY DATA:
Laboratory studies show: - White blood cell count: 12,000/mm³ (normal 4,500-11,000) - Hemoglobin: 13.2 g/dL (normal 12-16) - Platelets: 250,000/mm³ (normal 150,000-450,000) - Aspartate aminotransferase (AST): 45 U/L (normal 10-40) - Alanine aminotransferase (ALT): 50 U/L (normal 7-56) - Alkaline phosphatase: 130 U/L (normal 40-130) - Total bilirubin: 1.2 mg/dL (normal 0.3-1.2)

IMAGING:
An abdominal ultrasound reveals a gallbladder with a calcified rim and a central bile-filled dark area.

## Encounter 1: source 2020-07-15, practice 2025-10-12, inpatient, General Surgery

Chief complaint: Intermittent cramping abdominal pain, nausea, vomiting, and obstipation for 2 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Intermittent cramping abdominal pain, nausea, vomiting, and obstipation for 2 days

HISTORY OF PRESENT ILLNESS:
This 66-year-old female with chronic cholecystitis who presented six months ago with acute cholecystitis presents with 2-day history of intermittent cramping abdominal pain, nausea, and vomiting. She reports that pain is worse after eating and has not passed stool or gas for last 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute cholecystitis (diagnosed 2020-01-15)
- Atrial fibrillation
- Hyperlipidemia
- Hypertension

Her past medical history includes chronic cholecystitis and atrial fibrillation.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy (2010)
- Right breast lumpectomy (2014)

MEDICATIONS:
She takes warfarin.

Home Medications:
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 10 mg daily

ALLERGIES:
Sulfa drugs (rash), Penicillin (anaphylaxis)

FAMILY HISTORY:
- Mother: Cholelithiasis with cholecystectomy age 68
- Father: Atrial fibrillation, hyperlipidemia, died of myocardial infarction age 72
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker (15 pack-years, quit 8 years ago). Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On examination, her abdomen is distended and tympanic to percussion with hyperactive bowel sounds

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 12,000/mm^3
- Platelets: 250,000/mm^3
- INR: 2.8
- Serum creatinine: 1.1 mg/dL
- Total bilirubin: 1.2 mg/dL
- Alkaline phosphatase: 120 U/L

IMAGING:
A plain abdominal X-ray shows air in the biliary tree and a radiopaque mass in the right lower quadrant
